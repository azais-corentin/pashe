use async_trait::async_trait;
use human_repr::HumanDuration;
use reqwest::{Response, StatusCode};
use reqwest_middleware::{Middleware, Next};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

// Represents the state of a single rate limit rule
#[derive(Debug, Clone)]
struct RuleState {
    remaining_hits: u32,
    reset_time: Instant,
}

// Custom middleware for handling the specific rate limiting logic
#[derive(Debug)]
pub struct RateLimitMiddleware {
    state: Arc<Mutex<HashMap<String, RuleState>>>,
    last_request_time: Arc<Mutex<Instant>>,
    shutdown_token: CancellationToken,
}

impl RateLimitMiddleware {
    pub fn new(shutdown_token: CancellationToken) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            last_request_time: Arc::new(Mutex::new(Instant::now())),
            shutdown_token,
        }
    }
}

impl Default for RateLimitMiddleware {
    fn default() -> Self {
        Self::new(CancellationToken::new())
    }
}

#[async_trait]
impl Middleware for RateLimitMiddleware {
    #[tracing::instrument(skip_all, level = "trace")]
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        // Proactive check: Wait if we know we are near the limit
        let wait_duration = {
            let mut state_map = self.state.lock().unwrap();
            let now = Instant::now();

            // Clear expired rules and find the longest wait time needed
            state_map.retain(|_, state| state.reset_time > now);
            state_map
                .values()
                .filter_map(|state| {
                    if state.remaining_hits == 0 {
                        // If no hits left, we must wait for the window to reset
                        Some(state.reset_time.saturating_duration_since(now))
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or_default()
        }; // Lock is released here

        if !wait_duration.is_zero() {
            tracing::warn!(
                "Proactive rate limit: waiting for {}",
                wait_duration.human_duration()
            );
            tokio::select! {
                _ = tokio::time::sleep(wait_duration) => {},
                _ = self.shutdown_token.cancelled() => {
                    tracing::info!("Proactive rate limit sleep interrupted by shutdown");
                    return Err(reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                        "Rate limit sleep interrupted by shutdown"
                    )));
                }
            }
        }

        *self.last_request_time.lock().unwrap() = Instant::now();

        let mut retries = 3; // Max retries
        let mut res;

        loop {
            // We need to clone the request, in case we need to retry it.
            // This will fail if the request body is a stream.
            let req_clone = req.try_clone().ok_or_else(|| {
                reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                    "Request body is not cloneable, cannot retry"
                ))
            })?;

            res = next.clone().run(req_clone, extensions).await?;

            // Reactive check: handle 429 and update state from headers
            if res.status() == StatusCode::TOO_MANY_REQUESTS && retries > 0 {
                retries -= 1;
                if let Some(retry_after) = res.headers().get("Retry-After") {
                    if let Ok(seconds) = retry_after.to_str().unwrap_or("5").parse::<u64>() {
                        let wait_duration = Duration::from_secs(seconds);
                        tracing::warn!(
                            "Reactive rate limit (429): waiting for {} before retrying. Retries left: {}",
                            wait_duration.human_duration(),
                            retries
                        );
                        tokio::select! {
                            _ = tokio::time::sleep(wait_duration) => {},
                            _ = self.shutdown_token.cancelled() => {
                                tracing::info!("Reactive rate limit sleep interrupted by shutdown");
                                return Err(reqwest_middleware::Error::Middleware(anyhow::anyhow!(
                                    "Rate limit sleep interrupted by shutdown"
                                )));
                            }
                        }
                        // Continue to the next iteration of the loop to retry
                        continue;
                    }
                }
                // If Retry-After is not present, break and return the 429 response
                break;
            }

            // If the status is not 429, or we are out of retries, break the loop
            break;
        }

        // Update state from successful response headers
        if let Some(rules_header) = res.headers().get("X-Rate-Limit-Rules") {
            let rules = rules_header.to_str().unwrap_or("").split(',');
            let mut state_map = self.state.lock().unwrap();

            for rule in rules.filter(|r| !r.is_empty()) {
                let limit_key = format!("X-Rate-Limit-{rule}");
                let state_key = format!("X-Rate-Limit-{rule}-State");

                if let (Some(limit_val), Some(state_val)) =
                    (res.headers().get(&limit_key), res.headers().get(&state_key))
                {
                    // Parse "max_hits:period:ban_time"
                    let limit_parts: Vec<u64> = limit_val
                        .to_str()
                        .unwrap_or("")
                        .split(':')
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    // Parse "current_hits:period:active_ban"
                    let state_parts: Vec<u64> = state_val
                        .to_str()
                        .unwrap_or("")
                        .split(':')
                        .filter_map(|s| s.parse().ok())
                        .collect();

                    if limit_parts.len() == 3 && state_parts.len() == 3 {
                        let max_hits = limit_parts[0] as u32;
                        let period_secs = limit_parts[1];
                        let current_hits = state_parts[0] as u32;

                        let new_state = RuleState {
                            remaining_hits: max_hits.saturating_sub(current_hits),
                            reset_time: *self.last_request_time.lock().unwrap()
                                + Duration::from_secs(period_secs),
                        };

                        state_map.insert(rule.to_string(), new_state);
                    }
                }
            }
        }
        Ok(res)
    }
}
