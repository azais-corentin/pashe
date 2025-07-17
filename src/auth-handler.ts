import { getLogger } from "@logtape/logtape";
import type { RedisClientType } from "redis";

const logger = getLogger(["pashe", "auth-handler"]);

export const retrieveToken = async (
    cache: RedisClientType,
    client_id: string,
    client_secret: string,
) => {
    // Check if token was already retrieved
    const tokenRetrieved = await cache.exists("token");

    if (tokenRetrieved === 0) {
        logger.info("Fetching new token");

        const scope = "service:psapi";

        const data = new URLSearchParams();
        data.append("client_id", client_id);
        data.append("client_secret", client_secret);
        data.append("grant_type", "client_credentials");
        data.append("scope", scope);

        const oauthResponse = await fetch("https://www.pathofexile.com/oauth/token", {
            method: "POST",
            body: data,
            headers: {
                "Content-Type": "application/x-www-form-urlencoded",
            },
        });

        const oauth = (await oauthResponse.json()) as {
            access_token?: string;
            error_description?: string;
        };

        if (!oauth.access_token) {
            logger.error(`Error fetching token: ${oauth.error_description}`);
            return "undefined";
        }

        logger.debug("Retrieved new token {oauth}", { oauth });

        await cache.set("token", oauth.access_token, {
            expiration: {
                type: "EX",
                value: 60 * 60 * 24 * 28, // 28 days
            },
        });

        return oauth.access_token;
    } else {
        logger.info(`Using cached token`);

        return (await cache.get("token")) ?? "undefined";
    }
};
