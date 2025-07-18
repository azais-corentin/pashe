import { getLogger } from "@logtape/logtape";

const logger = getLogger(["pashe", "api", "ninja"]);

export const fetchInitialNextChangeId = async (): Promise<string> => {
    const response = await fetch("https://poe.ninja/api/data/getstats");

    if (!response.ok) {
        logger.error(`Failed to fetch initial next change ID: HTTP ${response.status}`, {
            status: response.status,
            body: await response.text(),
        });
        throw new Error(`Failed to fetch initial next change ID: ${response.statusText}`);
    }

    const data = (await response.json()) as { next_change_id?: string };
    if (!data.next_change_id) {
        logger.error("next_change_id is missing in response", { data });
        throw new Error("next_change_id is missing in response");
    }
    return data.next_change_id;
};
