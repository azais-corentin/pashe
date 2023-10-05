import { retrieveToken } from "AuthHandler";
import { RedisFlushModes, createClient } from "redis";
import { InfluxDB, Point, HttpError } from '@influxdata/influxdb-client'


import { RateLimitedHandler } from "Api/RateLimit";
import { GetPublicStashes } from "Api/PublicStash";
import { type } from "os";
import { exit } from "process";
import { sleep } from "bun";

// Retrieve client id/secret from environment
const client_id = process.env.CLIENT_ID;
const client_secret = process.env.CLIENT_SECRET;

// Connect to redis
const tokenCache = await createClient({
    url: "redis://redis",
    database: 0
})
    .on('error', err => () => {
        console.log('Redis Client Error', err)
        throw err;
    })
    .connect();

const hashCache = await createClient({
    url: "redis://redis",
    database: 1
})
    .on('error', err => () => {
        console.log('Redis Client Error', err)
        throw err;
    })
    .connect();

const token = await retrieveToken(tokenCache, client_id, client_secret);

const handler = new RateLimitedHandler(token);

let next_change_id = "2116708015-2110081184-2041061893-2265422455-2197472551";

// let totalStashes = 0;

// let totalDivinePrice = 0;
// let totalDivineListings = 0;


const start = performance.now();

interface ItemValue {
    value: number;
    currency: string;
}

const extractNoteValue = (note: string): ItemValue | undefined => {
    const priceTokens = note.split(" ");

    if (priceTokens.length < 3) {
        return;
    }

    if (priceTokens[0] != "~price") {
        return;
    }

    const value = (() => {
        const priceFraction = priceTokens[1].split("/")
        if (priceFraction.length > 1) {
            return parseInt(priceFraction[0]) / parseInt(priceFraction[1]);
        } else {
            return parseInt(priceFraction[0]);
        }
    })();

    if (isNaN(value)) {
        return;
    }

    return { value, currency: priceTokens[2] };
}

// const typeMap = new Map<string, BigInt>;

// hashCache.tDigest.create("test")

const writeApi = new InfluxDB({ url: process.env.INFLUX_URL ?? "", token: process.env.INFLUX_TOKEN }).getWriteApi(process.env.INFLUX_ORG ?? "", process.env.INFLUX_BUCKET ?? "", 'ns')

for (let i = 0; i < 100; i++) {
    const point1 = new Point("temperature").floatField("value", 20 + Math.round(100 * Math.random()) / 10).tag("source", "test");
    await sleep(1);

    writeApi.writePoint(point1);
}

await writeApi.close();

const queryApi = new InfluxDB({ url: process.env.INFLUX_URL ?? "", token: process.env.INFLUX_TOKEN }).getQueryApi(process.env.INFLUX_ORG ?? "");

for await (const { values, tableMeta } of queryApi.iterateRows('from(bucket:"pashe") |> range(start: -1d) |> filter(fn: (r) => r._measurement == "temperature")')) {
    const o = tableMeta.toObject(values)

    console.log(
        `${o._time} ${o._measurement} in '${o.location}' (${o.example}): ${o._field}=${o._value}`
    )
}

exit(0);

while (true) {
    // console.log(`Fetching ${next_change_id}`);
    const public_stashes = await GetPublicStashes(handler, next_change_id);

    for (const stash of public_stashes.stashes) {
        const stashValue = extractNoteValue(stash.stash ?? "");

        for (const item of stash.items) {
            let itemValue: ItemValue | undefined;
            if (item.note === undefined) {

                // console.log(`No item note; stash name: ${stash.stash}`);

                itemValue = stashValue;
            } else {
                itemValue = extractNoteValue(item.note);
            }

            if (itemValue !== undefined) {
                // if (await hashCache.exists(item.baseType)) {
                //     if (BigInt(await hashCache.get(item.baseType) ?? "") != BigInt(Bun.hash(item.baseType))) {
                //         console.log("Hash is UNSTABLE!!");
                //     }
                // } else {
                //     hashCache.set(item.baseType, String(Bun.hash(item.baseType)));
                // }

                // if (typeMap.has(item.baseType)) {
                //     if (typeMap.get(item.baseType) != BigInt(Bun.hash(item.baseType))) {

                //     }
                // } else {
                //     typeMap.set(item.baseType, BigInt(Bun.hash(item.baseType)));
                // }

                // console.log(`${item.baseType} is worth ${itemValue?.value} ${itemValue?.currency.toLocaleLowerCase()}`);
            }

            // // console.log(`name: ${item.name} / baseType: ${item.baseType}`);
            // if ("Divine Orb".localeCompare(item.baseType, 'en', { sensitivity: 'base' }) == 0) {
            //     const priceTokens = item.note.split(" ");
            //     if (priceTokens[0] != "~price" || priceTokens[2] != "chaos") {
            //         continue;
            //     }

            //     const price = (() => {
            //         const priceFraction = priceTokens[1].split("/")
            //         if (priceFraction.length > 1) {
            //             return parseInt(priceFraction[0]) / parseInt(priceFraction[1]);
            //         } else {
            //             return parseInt(priceFraction[0]);
            //         }
            //     })();

            //     if (isNaN(price)) {
            //         continue;
            //     }

            //     console.log(`Price ${price} (${item.note})`);

            //     totalDivinePrice += price;
            //     totalDivineListings++;
            // }
        }
    }

    // console.log(`Average divine price: ${totalDivinePrice / totalDivineListings} chaos/divine`);

    // const totalTime = performance.now() - start;
    // totalStashes += public_stashes.stashes.length;
    // console.log(`${totalStashes / (totalTime / 1000)} stashes/s`);

    next_change_id = public_stashes.next_change_id;
}

await tokenCache.disconnect();