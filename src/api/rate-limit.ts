import { getLogger } from "@logtape/logtape";
import { sleep } from "bun";
import { buildUrl, constants } from "./types";

const logger = getLogger(["pashe", "api", "rate-limit"]);

/**
 * Handles HTTP 429 Too Many Requests errors after they've happened
 */
export const postFetchHandler = async (
    url: string | Request | URL,
    init?: RequestInit | undefined,
    retry = 0,
): Promise<Response> => {
    if (retry > 5) {
        logger.error(`Fetch failed on endpoint {url} after 5 retries`, { url });
        // Handle recursive retry
        throw "Fetch failed";
    }

    const response = await fetch(url, init);

    if (response.status === 429) {
        const retryAfter = 1 + parseInt(response.headers.get("Retry-After") ?? "0");
        logger.warn("Rate limit exceeded, retrying in {retryAfter}s", { retryAfter });

        await sleep(retryAfter * 1000);

        return postFetchHandler(url, init, retry + 1);
    } else {
        return response;
    }
};

export const rateLimitedFetch = (url: string | Request | URL, init?: RequestInit | undefined) => {
    return postFetchHandler(url, init);
};

const deepMergeWithSpread = <T extends Record<string, unknown>>(obj1: T, obj2: T): T => {
    const result = { ...obj1 };

    for (const key in obj2) {
        if (Object.hasOwn(obj2, key)) {
            if (obj2[key] instanceof Object && obj1[key] instanceof Object) {
                result[key] = deepMergeWithSpread(obj1[key] as T, obj2[key] as T) as T[Extract<
                    keyof T,
                    string
                >];
            } else {
                result[key] = obj2[key];
            }
        }
    }

    return result;
};

export class RateLimitedHandler {
    defaultOptions: RequestInit;

    private resetDate = -1; // Date when the rate limit resets
    private remainingRequests = 1; // Remaining requests that can be made before we are rate limited
    private limitRequests = Number.POSITIVE_INFINITY; // Number of requests that can be made before we are rate limited
    private oldLimitRequests = Number.POSITIVE_INFINITY; // Change detection

    public constructor(token: string) {
        this.defaultOptions = deepMergeWithSpread(constants.defaultOptions, {
            headers: {
                Authorization: `Bearer ${token}`,
            },
        });
        logger.debug(
            "RateLimitedHandler initialized with options {*}",
            this.defaultOptions as never,
        );
    }

    private get limited(): boolean {
        return this.remainingRequests <= 0 && this.resetDate > Date.now();
    }

    private get timeToReset(): number {
        return this.resetDate - Date.now();
    }

    public async fetch(
        endpoint: string,
        init?: RequestInit | undefined,
        retry = 0,
    ): Promise<Response> {
        if (retry > 5) {
            logger.error(`Fetch failed on endpoint ${endpoint} after 5 retries`);
            throw new Error(`Fetch failed on endpoint ${endpoint} after 5 retries`);
        }

        const fetchOptions = { ...this.defaultOptions, ...init };
        const url = buildUrl(endpoint);

        while (this.limited) {
            // We were previously limited due to:
            // - normal rate limiting
            // - status 429 too many requests

            await sleep(this.timeToReset);
        }

        const response = await fetch(url, fetchOptions);

        // Extract up to date limits
        const rateLimitRule = response.headers.get("X-Rate-Limit-Ip")?.split(":");
        const rateLimitState = response.headers.get("X-Rate-Limit-Ip-State")?.split(":");

        // Update local limit values
        this.limitRequests = rateLimitRule ? Number(rateLimitRule[0]) : Number.POSITIVE_INFINITY;
        this.remainingRequests = rateLimitState
            ? this.limitRequests - Number(rateLimitState[0])
            : 1;
        this.resetDate = rateLimitRule ? Number(rateLimitRule[1]) * 1000 + Date.now() : Date.now();

        logger.debug("Updated rate limits {*}", {
            remaining: this.remainingRequests,
            limit: this.limitRequests,
            resetTimeMs: this.timeToReset,
        });

        // Check if we got rate limited anyways
        if (response.status === 429) {
            const retryAfter = 1 + parseInt(response.headers.get("Retry-After") ?? "0");
            logger.warn("Unexpectedly rate limited, retrying in {retryAfter}s", { retryAfter });

            // Fetch up to date next retry date
            this.remainingRequests = 0;
            this.resetDate = Date.now() + retryAfter * 1000;

            return this.fetch(url, fetchOptions, retry++);
        }

        return response;
    }
}
