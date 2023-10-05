import { build, sleep } from "bun";
import { buildUrl } from "Api/POE";

/**
 * Handles HTTP 429 Too Many Requests errors after they've happened
 */
export const postFetchHandler = async (url: string | Request | URL, init?: RequestInit | undefined, retry = 0): Promise<Response> => {
    if (retry > 5) { // Handle recursive retry
        throw 'Fetch failed';
    }

    const response = await fetch(url, init);

    if (response.status == 429) {
        const retryAfter = 1 + parseInt(response.headers.get("Retry-After") ?? "0");
        console.log(`Retrying in ${retryAfter}s`);

        await sleep(retryAfter * 1000);

        return postHandler(url, init, retry + 1);
    } else {
        return response;
    }
};

export const rateLimitedFetch = (url: string | Request | URL, init?: RequestInit | undefined) => {
    return postFetchHandler(url, init);
}

export class RateLimitedHandler {
    defaultOptions: any;

    private resetDate = -1; // Date when the rate limit resets
    private remainingRequests = 1; // Remaining requests that can be made before we are rate limited
    private limitRequests = Number.POSITIVE_INFINITY; // Number of requests that can be made before we are rate limited
    private oldLimitRequests = Number.POSITIVE_INFINITY; // Change detection

    public constructor(token: string) {
        this.defaultOptions = {
            headers: {
                "Authorization": `Bearer ${token}`,
                "User-Agent": "OAuth pashebackend/0.1 (contact: haellsigh@gmail.com)"
            }
        };
    }

    private get limited(): boolean {
        return this.remainingRequests <= 0 && this.resetDate > Date.now();
    }

    private get timeToReset(): number {
        return this.resetDate - Date.now();
    }

    public async fetch(endpoint: string, init?: RequestInit | undefined, retry = 0): Promise<Response | undefined> {
        if (retry > 5) {
            console.error(`Fetch failed on endpoint ${endpoint} after 5 retries`);
            return;
        }

        const fetchOptions = { ...this.defaultOptions, ...init };
        const url = buildUrl(endpoint);

        while (this.limited) {
            // We were previously limited due to:
            // - normal rate limiting
            // - status 429 too many requests
            // console.log(`Waiting ${this.timeToReset / 1000}s for next reset`);
            await sleep(this.timeToReset);
        }

        const response = await fetch(url, fetchOptions);

        // Extract up to date limits
        const rateLimitRule = response.headers.get("X-Rate-Limit-Ip")?.split(":");
        const rateLimitState = response.headers.get("X-Rate-Limit-Ip-State")?.split(":");

        // Update local limit values
        this.limitRequests = rateLimitRule ? Number(rateLimitRule[0]) : Number.POSITIVE_INFINITY;
        this.remainingRequests = rateLimitState ? this.limitRequests - Number(rateLimitState[0]) : 1;
        this.resetDate = rateLimitRule ? Number(rateLimitRule[1]) * 1000 + Date.now() : Date.now();

        // Log
        // console.debug(`${this.remainingRequests}/${this.limitRequests} remaining, reset in ${this.timeToReset}ms`);

        // Check if we got rate limited anyways
        if (response.status == 429) {
            console.warn("Unexpected rate limiting..!");

            const resetDate = response.headers.get("Retry-After");

            // Fetch up to date next retry date
            this.remainingRequests = 0;
            this.resetDate = resetDate ? Number(resetDate) * 1000 + Date.now() : Date.now();

            return this.fetch(url, fetchOptions, retry++);
        }

        return response;
    }
}