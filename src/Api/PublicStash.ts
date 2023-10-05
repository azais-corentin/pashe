import { buildUrl, type PublicStashStream } from "Api/POE";
import { type RateLimitedHandler } from "Api/RateLimit";


export const GetPublicStashes = async (handler: RateLimitedHandler, nextChangeId = "0"): Promise<PublicStashStream> => {
    const response = await handler.fetch(`public-stash-tabs?id=${nextChangeId}`);

    if (response === undefined) {
        console.error("Errore");
        throw new Error("fetch failed");
    }

    if (response.status == 200) {
        return response.json();
    }

    throw new Error(`status ${response.status}`);
};

export default GetPublicStashes;