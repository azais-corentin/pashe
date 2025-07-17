import { exit } from "node:process";
import { configure, getConsoleSink, getLogger } from "@logtape/logtape";
import { getPrettyFormatter } from "@logtape/pretty";
import { getSentrySink } from "@logtape/sentry";
import * as Sentry from "@sentry/bun";
import { createClient } from "redis";
import { GetPublicStashes } from "./api/public-stash";
import { RateLimitedHandler } from "./api/rate-limit";
import { retrieveToken } from "./auth-handler";

const sentryClient = Sentry.init({
    dsn: process.env.SENTRY_DSN,
    _experiments: { enableLogs: true },
});

await configure({
    sinks: {
        sentry: getSentrySink(sentryClient),
        /*
        console: getConsoleSink({
            formatter: getPrettyFormatter({
                timestamp: "date-time",
                icons: false,
                categorySeparator: ".",
            }),
        }),
        */
    },
    loggers: [{ category: [], sinks: ["sentry"], lowestLevel: "debug" }],
});

const logger = getLogger(["pashe", "main"]);

// Retrieve client id/secret from environment
const client_id = process.env.CLIENT_ID;
const client_secret = process.env.CLIENT_SECRET;

logger.info("Uhhh...");
logger.error("Database connection established", {
    host: "dbHost",
    port: "dbPort",
    username: "dbUser",
    loginTime: new Date(),
});

await Sentry.flush();

// Connect to redis
const tokenCache = await createClient({
    url: "redis://redis",
    database: 0,
})
    .on("error", (err) => {
        logger.error("Redis Client Error", err);
        throw err;
    })
    .connect();

const hashCache = await createClient({
    url: "redis://redis",
    database: 1,
})
    .on("error", (err) => {
        logger.error("Redis Client Error", err);
        throw err;
    })
    .connect();

if (!client_id || !client_secret) {
    logger.error("CLIENT_ID and CLIENT_SECRET environment variables are required");
    exit(-1);
}

const token = await retrieveToken(tokenCache, client_id, client_secret);

if (token === "undefined") {
    exit(-1);
}

interface ItemValue {
    value: number;
    currency: string;
}

const extractNoteValue = (note: string | null | undefined): ItemValue | undefined => {
    if (note == null) {
        return;
    }

    const priceTokens = note.split(" ");

    if (priceTokens.length < 3 || priceTokens[0] !== "~price") {
        return;
    }

    const value = (() => {
        const firstPriceToken = priceTokens[1];
        if (!firstPriceToken) {
            return NaN;
        }

        const priceFraction = firstPriceToken.split("/");

        // TODO: Strip any char that's not a number from price strings

        const numerator = parseInt(priceFraction[0] ?? "error");
        if (priceFraction.length > 1) {
            const denominator = parseInt(priceFraction[1] ?? "error");
            if (denominator > 0) {
                return numerator / denominator;
            } else {
                return numerator;
            }
        } else {
            return numerator;
        }
    })();

    if (Number.isNaN(value)) {
        return;
    }

    if (!Number.isFinite(value)) {
        return;
    }

    return { value, currency: priceTokens[2]?.toLocaleLowerCase() ?? "error" };
};

/*
const writeApi = new InfluxDB({
    url: process.env.INFLUX_URL ?? "",
    token: process.env.INFLUX_TOKEN,
}).getWriteApi(process.env.INFLUX_ORG ?? "", process.env.INFLUX_BUCKET ?? "", "ms");
*/

let next_change_id = "2135721132-2128260640-2059181769-2285909364-2218124765";
const handler = new RateLimitedHandler(token);

while (true) {
    console.log(`Fetching ${next_change_id}`);
    const public_stashes = await GetPublicStashes(handler, next_change_id);

    // const points: Point[] = [];
    let itemIndex = 0;

    for (const stash of public_stashes.stashes) {
        const stashValue = extractNoteValue(stash.stash);

        for (const item of stash.items) {
            const itemValue = item.note != null ? extractNoteValue(item.note) : stashValue;

            if (itemValue !== undefined) {
                /*
                let point = new Point("price")
                    .floatField("value", itemValue.value)
                    .stringField("", "")
                    .tag("currency", itemValue.currency)
                    .tag("baseType", item.baseType)
                    .uintField("itemIndex", itemIndex);
                if (item.typeLine.length > 1) {
                    point = point.tag("typeLine", item.typeLine);
                }
                points.push(point);
                */
                itemIndex++;
            }
        }
    }

    // Write points to influxdb
    // writeApi.writePoints(points);
    logger.info(`Wrote ${/*points.length*/ 0} points to database`);

    next_change_id = public_stashes.next_change_id;
}

await Promise.allSettled([/*writeApi.close(),*/ tokenCache.close()]);
