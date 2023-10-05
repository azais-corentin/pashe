import type { RedisClientType } from 'redis'

export const retrieveToken = async (cache: any,
    client_id: string | undefined, client_secret: string | undefined) => {
    // Check if token was already retrieved
    const tokenRetrieved = await cache.exists("token");

    if (tokenRetrieved == 0) {
        console.log("Fetching new token");

        const scope = "service:psapi";

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

        const oauth = await oauthResponse.json();

        console.log(oauth);

        await cache.set("token", oauth.access_token);

        return oauth.access_token;
    } else {
        console.log(`Using cached token`);

        return await cache.get("token") ?? "undefined";
    }
};