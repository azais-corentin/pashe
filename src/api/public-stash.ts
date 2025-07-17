import type { RateLimitedHandler } from "./rate-limit";
import type { PublicStashStream } from "./types";

export const getPublicStashes = async (
    handler: RateLimitedHandler,
    nextChangeId = "0",
): Promise<PublicStashStream> => {
    const response = await handler.fetch(`public-stash-tabs?id=${nextChangeId}`);

    if (response === undefined) {
        console.error("Errore");
        throw new Error("fetch failed");
    }

    if (response.status === 200) {
        return (await response.json()) as PublicStashStream;
    }

    throw new Error(`status ${response.status}`);
};

export default getPublicStashes;
