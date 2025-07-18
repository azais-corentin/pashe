import { getLogger } from "@logtape/logtape";
import type { RateLimitedHandler } from "./rate-limit";
import { ItemSchema, type PublicStashStream, PublicStashStreamSchema } from "./types";

const logger = getLogger(["pashe", "api", "rate-limit"]);

export const getPublicStashes = async (
    handler: RateLimitedHandler,
    nextChangeId = "0",
): Promise<PublicStashStream> => {
    const response = await handler.fetch(`public-stash-tabs?id=${nextChangeId}`);

    if (response.status !== 200) {
        logger.error("Failed to fetch public stashes, status {status}", {
            status: response.status,
        });
        throw new Error(`Failed to fetch public stashes, status ${response.status}`);
    }

    const json = await response.json();

    const start = performance.now();
    const publicStashStream = PublicStashStreamSchema.parse(json);
    const end = performance.now();
    logger.info("Parsed public stash stream in {duration}ms", {
        duration: (end - start).toFixed(2),
    });

    return publicStashStream;
};

export default getPublicStashes;
