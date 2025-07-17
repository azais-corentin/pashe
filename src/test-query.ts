import { InfluxDB, Point } from "@influxdata/influxdb-client";
import { configure, getConsoleSink, getLogger } from "@logtape/logtape";

await configure({
    sinks: { console: getConsoleSink() },
    loggers: [{ category: "pashe", lowestLevel: "debug", sinks: ["console"] }],
});

const logger = getLogger(["pashe", "test-query"]);

const queryApi = new InfluxDB({
    url: process.env.INFLUX_URL ?? "",
    token: process.env.INFLUX_TOKEN,
}).getQueryApi(process.env.INFLUX_ORG ?? "");

const fluxQuery = `from(bucket:"pashe")
      |> range(start: -1d)
      |> filter(fn: (r) => r["_measurement"] == "price")
      |> filter(fn: (r) => r["_field"] == "value")
      |> filter(fn: (r) => r["baseType"] == "Honoured Tattoo of the Oak")
      |> filter(fn: (r) => r["currency"] == "chaos")`;

for await (const { values, tableMeta } of queryApi.iterateRows(fluxQuery)) {
    const o = tableMeta.toObject(values);

    logger.info(
        `${o._time} ${o._measurement} in '${o.baseType}' (${o.currency}): ${o._field}=${o._value}`,
    );
}