import { getLogger } from "@logtape/logtape";
import { drizzle } from "drizzle-orm/node-postgres";

const logger = getLogger(["pashe", "db"]);

if (!process.env.DATABASE_URL) {
    logger.error("DATABASE_URL environment variable is required");
    process.exit(-1);
}

export const db = drizzle({ connection: process.env.DATABASE_URL, casing: "snake_case" });
