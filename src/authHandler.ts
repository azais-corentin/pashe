import { createClient } from "redis";

export const retrieveToken = async (client_id: string | undefined,
    client_secret: string | undefined) => {

    // Connect to redis
    const cache = await createClient({
        url: "redis://database"
    })
        .on('error', err => () => {
            console.log('Redis Client Error', err)
            throw err;
        })
        .connect();

    // Check if token was already retrieved
    const tokenRetrieved = await cache.exists("token");

    const token = await (async (): Promise<string> => {
        if (tokenRetrieved == 0) {
            console.log("Fetching new token");

            const scope = "service:psapi service:leagues";

            const data = new URLSearchParams();
            data.append("client_id", client_id ?? "undefined");
            data.append("client_secret", client_secret ?? "undefined");
            data.append("grant_type", "client_credentials");
            data.append("scope", scope ?? "undefined");

            const oauthResponse = await fetch("https://www.pathofexile.com/oauth/token", {
                method: "POST",
                body: data,
                headers: { "Content-Type": "application/x-www-form-urlencoded" },
            });

            const body = await oauthResponse.json();

            await cache.set("token", body.access_token);

            return body.access_token;
        } else {
            console.log(`Fetching cached token`);

            return await cache.get("token") ?? "undefined";
        }
    })();

    await cache.disconnect();

    return token;
}