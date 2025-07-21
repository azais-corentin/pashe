use anyhow::{Context, Result};
use http::{Extensions, HeaderMap};
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct RateLimits {
    pub remaining_requests: u32,
    pub reset_date: tokio::time::Instant,
    pub period: u32,
}

/// A simple rate limiter that is sufficient for our use case.
/// It only works well if there's only one request in transit at a time.
trait RateLimiter {
    async fn ensure(&mut self);
    fn update(&mut self, response: &Response) -> Result<()>;
}

fn split_header_to_vec<'a>(headers: &'a HeaderMap, header_name: &str) -> Result<Vec<&'a str>> {
    Ok(headers
        .get(header_name)
        .ok_or_else(|| anyhow::anyhow!("{} header not found", header_name))?
        .to_str()?
        .split(':')
        .collect())
}

impl RateLimiter for RateLimits {
    async fn ensure(&mut self) {
        let limited = self.remaining_requests == 0;

        if limited {
            debug!(
                "Sleep required to avoid rate limit, sleeping until {}",
                chrono::Utc::now()
                    + chrono::Duration::from_std(
                        self.reset_date
                            .saturating_duration_since(tokio::time::Instant::now()),
                    )
                    .unwrap()
            );

            // If there's no reset date, update it from the period and set it to handle the worst case
            if self.reset_date <= tokio::time::Instant::now() {
                debug!("Resetting reset_date to now + period");
                self.reset_date = tokio::time::Instant::now()
                    + tokio::time::Duration::from_secs(self.period as u64);
            }

            tokio::time::sleep_until(self.reset_date).await;
            debug!("Finished sleeping, resuming request");
        }

        self.remaining_requests = self.remaining_requests.saturating_sub(1);
    }

    fn update(&mut self, response: &Response) -> Result<()> {
        let headers = response.headers();

        if !headers.contains_key("X-Rate-Limit-Ip")
            || !headers.contains_key("X-Rate-Limit-Ip-State")
        {
            warn!("Response headers do not contain rate limit information, skipping update");
            return Ok(());
        }

        let ip_rate_limits = split_header_to_vec(headers, "X-Rate-Limit-Ip")?;
        let ip_rate_limits_state = split_header_to_vec(headers, "X-Rate-Limit-Ip-State")?;

        if ip_rate_limits.len() != 3 {
            return Err(anyhow::anyhow!(
                "X-Rate-Limit-Ip header has invalid format '{}'",
                ip_rate_limits.join(":")
            ));
        }
        if ip_rate_limits_state.len() != 3 {
            return Err(anyhow::anyhow!(
                "X-Rate-Limit-Ip-State header has invalid format '{}'",
                ip_rate_limits_state.join(":")
            ));
        }

        let max_hits = ip_rate_limits[0]
            .parse::<u32>()
            .context("Failed to parse max hits from X-Rate-Limit-Ip header")?;
        let period = ip_rate_limits[1]
            .parse::<u32>()
            .context("Failed to parse period from X-Rate-Limit-Ip header")?;

        let current_hit_count = ip_rate_limits_state[0]
            .parse::<u32>()
            .context("Failed to parse current hit count from X-Rate-Limit-Ip-State header")?;

        self.reset_date =
            tokio::time::Instant::now() + tokio::time::Duration::from_secs(period as u64);

        self.remaining_requests = max_hits - current_hit_count;
        self.period = period;

        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            warn!("Rate limit exceeded!");
            self.remaining_requests = 0;
            let restricted_time = ip_rate_limits_state[2]
                .parse::<u32>()
                .context("Failed to parse restricted time from X-Rate-Limit-Ip-State header")?;
            self.reset_date = tokio::time::Instant::now()
                + tokio::time::Duration::from_secs(restricted_time as u64);
        }

        Ok(())
    }
}

pub struct RateLimitingMiddleware {
    pub limits: tokio::sync::Mutex<RateLimits>,
}

impl Default for RateLimits {
    fn default() -> Self {
        Self {
            remaining_requests: u32::MAX,
            reset_date: tokio::time::Instant::now(),
            period: 0,
        }
    }
}

impl Default for RateLimitingMiddleware {
    fn default() -> Self {
        Self {
            limits: tokio::sync::Mutex::new(RateLimits::default()),
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimitingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        self.limits.lock().await.ensure().await;
        let response = next.run(req, extensions).await;

        if let Ok(response) = response.as_ref() {
            self.limits.lock().await.update(response)?;
        }

        response
    }
}
