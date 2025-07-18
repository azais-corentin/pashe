import { exit } from "node:process";
import { configure, getConsoleSink, getLogger } from "@logtape/logtape";
import { getPrettyFormatter } from "@logtape/pretty";
import { createClient, type RedisClientType } from "redis";
import { fetchInitialNextChangeId } from "./api/ninja";
import { getPublicStashes } from "./api/public-stash";
import { RateLimitedHandler } from "./api/rate-limit";
import { retrieveToken } from "./auth-handler";
import { db } from "./db/db";
import { items, publicStashChanges, publicStashChangesToItems } from "./db/schema";

await configure({
    sinks: {
        console: getConsoleSink({
            formatter: getPrettyFormatter({
                timestamp: "date-time",
                icons: false,
                categorySeparator: ".",
            }),
        }),
    },
    loggers: [
        { category: ["logtape", "meta"], sinks: ["console"], lowestLevel: "warning" },
        { category: [], sinks: ["console"], lowestLevel: "debug" },
    ],
});

const logger = getLogger(["pashe", "main"]);

const client_id = process.env.CLIENT_ID;
const client_secret = process.env.CLIENT_SECRET;

if (!client_id || !client_secret) {
    logger.error("CLIENT_ID and CLIENT_SECRET environment variables are required");
    exit(-1);
}

const cache: RedisClientType = await createClient({
    url: "redis://redis",
    database: 0,
});

cache
    .on("error", (err) => {
        logger.error("Redis error: {err}", err);
        throw err;
    })
    .connect();

/*
const hashCache = await createClient({
    url: "redis://redis",
    database: 1,
})
    .on("error", (err) => {
        logger.error("Redis Client Error", err);
        throw err;
    })
    .connect();
*/

const token = await retrieveToken(cache, client_id, client_secret);

if (token === "undefined") {
    exit(-1);
}

const updateNextChangeId = async (nextChangeId: string) => {
    await cache.set("next_change_id", nextChangeId, {
        expiration: {
            type: "EX",
            value: 60 * 5, // Cache for 5 minutes
        },
    });
    logger.debug("Updated next_change_id in cache: {next_change_id}", {
        next_change_id: nextChangeId,
    });
};

const getInitialNextChangeId = async (): Promise<string> => {
    let initialChangeId = await cache.get("next_change_id");

    if (initialChangeId) {
        logger.info("Using cached next_change_id: {next_change_id}", {
            next_change_id: initialChangeId,
        });
        return initialChangeId;
    }

    logger.debug("No cached next_change_id found, fetching initial value from API");

    initialChangeId = await fetchInitialNextChangeId();
    updateNextChangeId(initialChangeId);

    logger.info("Fetched initial next_change_id: {next_change_id}", {
        next_change_id: initialChangeId,
    });

    return await fetchInitialNextChangeId();
};

let nextChangeId = await getInitialNextChangeId();
const handler = new RateLimitedHandler(token);

while (true) {
    logger.info(`Fetching public stashes with change id {next_change_id}`, {
        next_change_id: nextChangeId,
    });
    const public_stashes = await getPublicStashes(handler, nextChangeId);

    logger.debug(`Fetched {count} public stashes`, { count: public_stashes.stashes.length });

    for (const stash of public_stashes.stashes) {
        // await db.insert(publicStashChanges).values({
        //     id: stash.id,
        //     public: stash.public,
        //     accountName: stash.accountName,
        //     stash: stash.stash,
        //     stashType: stash.stashType,
        //     league: stash.league,
        // });
        logger.debug("Inserted public stash change", { stashId: stash.id });

        const itemsToInsert = stash.items.filter((item) => item.id);
        if (itemsToInsert.length > 0) {
            await db
                .insert(items)
                .values(itemsToInsert as never)
                .onConflictDoNothing({
                    target: items.id,
                });
            logger.info(`Inserted ${itemsToInsert.length} items for stash ${stash.id}`);
        }

        /*
        for (const item of stash.items) {
            if (!item.id) {
                logger.warn("Skipping item without valid id", { item });
                continue;
            }
            await db.insert(items).values(item);
            await db.insert(publicStashChangesToItems).values({
                publicStashChangeId: stash.id,
                itemId: item.id,
            });
        }
        */
    }

    logger.info(`Wrote ${public_stashes.stashes.length} stashes to database`);

    nextChangeId = public_stashes.next_change_id;
    updateNextChangeId(nextChangeId);
}

await Promise.allSettled([/*writeApi.close(),*/ cache.close()]);
