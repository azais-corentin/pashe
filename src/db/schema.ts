import {
    type AnyPgColumn,
    boolean,
    integer,
    jsonb,
    pgEnum,
    pgTable,
    primaryKey,
    serial,
    text,
    varchar,
} from "drizzle-orm/pg-core";

export const displayModeEnum = pgEnum("display_mode", [
    "NameFollowedByValues",
    "ValuesFollowedByName",
    "ProgressBar",
    "ValuesInsertedIntoStringByIndex",
    "Separator",
]);

export const itemSocketAttrEnum = pgEnum("item_socket_attr", ["S", "D", "I", "G", "A", "DV"]);
export const itemSocketSColourEnum = pgEnum("item_socket_s_colour", [
    "R",
    "G",
    "B",
    "W",
    "A",
    "DV",
]);
export const itemSocketTypeEnum = pgEnum("item_socket_type", ["gem", "jewel", "rune"]);
export const itemSocketItemEnum = pgEnum("item_socket_item", [
    "emerald",
    "sapphire",
    "ruby",
    "rune",
    "soulcore",
    "primaltalisman",
    "vividtalisman",
    "wildtalisman",
    "sacredtalisman",
    "activegem",
    "supportgem",
]);

export const rarityEnum = pgEnum("rarity", ["Normal", "Magic", "Rare", "Unique"]);

export const items = pgTable("items", {
    realm: varchar("realm"),
    verified: boolean("verified").notNull(),
    w: integer("w").notNull(),
    h: integer("h").notNull(),
    icon: varchar("icon").notNull(),
    support: boolean("support"),
    stackSize: integer("stack_size"),
    maxStackSize: integer("max_stack_size"),
    stackSizeText: varchar("stack_size_text"),
    league: varchar("league"),
    id: varchar("id").primaryKey(),
    gemSockets: jsonb("gem_sockets"),
    influences: jsonb("influences"),
    elder: boolean("elder"),
    shaper: boolean("shaper"),
    searing: boolean("searing"),
    tangled: boolean("tangled"),
    memoryItem: boolean("memory_item"),
    abyssJewel: boolean("abyss_jewel"),
    delve: boolean("delve"),
    fractured: boolean("fractured"),
    synthesised: boolean("synthesised"),
    name: varchar("name").notNull(),
    typeLine: varchar("type_line").notNull(),
    baseType: varchar("base_type").notNull(),
    rarity: rarityEnum("rarity"),
    identified: boolean("identified").notNull(),
    itemLevel: integer("item_level"),
    unidentifiedTier: integer("unidentified_tier"),
    ilvl: integer("ilvl").notNull(),
    note: varchar("note"),
    forum_note: varchar("forum_note"),
    lockedToCharacter: boolean("locked_to_character"),
    lockedToAccount: boolean("locked_to_account"),
    duplicated: boolean("duplicated"),
    split: boolean("split"),
    corrupted: boolean("corrupted"),
    unmodifiable: boolean("unmodifiable"),
    cisRaceReward: boolean("cis_race_reward"),
    seaRaceReward: boolean("sea_race_reward"),
    thRaceReward: boolean("th_race_reward"),
    talismanTier: integer("talisman_tier"),
    secDescrText: varchar("sec_descr_text"),
    utilityMods: jsonb("utility_mods"),
    enchantMods: jsonb("enchant_mods"),
    runeMods: jsonb("rune_mods"),
    scourgeMods: jsonb("scourge_mods"),
    implicitMods: jsonb("implicit_mods"),
    explicitMods: jsonb("explicit_mods"),
    craftedMods: jsonb("crafted_mods"),
    fracturedMods: jsonb("fractured_mods"),
    crucibleMods: jsonb("crucible_mods"),
    cosmeticMods: jsonb("cosmetic_mods"),
    veiledMods: jsonb("veiled_mods"),
    veiled: boolean("veiled"),
    gemBackground: varchar("gem_background"),
    gemSkill: varchar("gem_skill"),
    descrText: varchar("descr_text"),
    flavourText: jsonb("flavour_text"),
    flavourTextParsed: jsonb("flavour_text_parsed"),
    flavourTextNote: varchar("flavour_text_note"),
    prophecyText: varchar("prophecy_text"),
    isRelic: boolean("is_relic"),
    foilVariation: integer("foil_variation"),
    replica: boolean("replica"),
    foreseeing: boolean("foreseeing"),
    ruthless: boolean("ruthless"),
    frameType: integer("frame_type"),
    artFilename: varchar("art_filename"),
    x: integer("x"),
    y: integer("y"),
    inventoryId: varchar("inventory_id"),
    socket: integer("socket"),
    colour: varchar("colour"),
    socketedInItemId: varchar("socketed_in_item_id").references((): AnyPgColumn => items.id),
});

export const itemProperties = pgTable("item_properties", {
    id: serial("id").primaryKey(),
    name: varchar("name").notNull(),
    values: jsonb("values").notNull(),
    displayMode: displayModeEnum("display_mode"),
    progress: integer("progress"),
    type: integer("type"),
    suffix: varchar("suffix"),
    icon: varchar("icon"),
    itemPropertyId: varchar("item_property_id").references(() => items.id),
    notablePropertyId: varchar("notable_property_id").references(() => items.id),
    requirementId: varchar("requirement_id").references(() => items.id),
    additionalPropertyId: varchar("additional_property_id").references(() => items.id),
    nextLevelRequirementId: varchar("next_level_requirement_id").references(() => items.id),
    grantedSkillId: varchar("granted_skill_id").references(() => items.id),
    hybridId: varchar("hybrid_id").references(() => items.id),
    gemPageId: varchar("gem_page_id").references(() => items.id),
});

export const rewards = pgTable("rewards", {
    id: serial("id").primaryKey(),
    label: varchar("label").notNull(),
    rewards: jsonb("rewards").notNull(),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const logbookMods = pgTable("logbook_mods", {
    id: serial("id").primaryKey(),
    name: varchar("name").notNull(),
    factionId: varchar("faction_id").notNull(),
    factionName: varchar("faction_name").notNull(),
    mods: jsonb("mods").notNull(),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const ultimatumMods = pgTable("ultimatum_mods", {
    id: serial("id").primaryKey(),
    type: varchar("type").notNull(),
    tier: integer("tier").notNull(),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const incubatedItems = pgTable("incubated_items", {
    id: serial("id").primaryKey(),
    name: varchar("name").notNull(),
    level: integer("level").notNull(),
    progress: integer("progress").notNull(),
    total: integer("total").notNull(),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const scourgedItems = pgTable("scourged_items", {
    id: serial("id").primaryKey(),
    tier: integer("tier").notNull(),
    level: integer("level"),
    progress: integer("progress"),
    total: integer("total"),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const crucibleNodes = pgTable("crucible_nodes", {
    id: serial("id").primaryKey(),
    skill: integer("skill"),
    tier: integer("tier"),
    icon: varchar("icon"),
    allocated: boolean("allocated"),
    isNotable: boolean("is_notable"),
    isReward: boolean("is_reward"),
    stats: jsonb("stats"),
    reminderText: jsonb("reminder_text"),
    orbit: integer("orbit"),
    orbitIndex: integer("orbit_index"),
    out: jsonb("out").notNull(),
    in: jsonb("in").notNull(),
});

export const crucible = pgTable("crucible", {
    id: serial("id").primaryKey(),
    layout: varchar("layout").notNull(),
    nodes: jsonb("nodes").notNull(),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const hybrid = pgTable("hybrid", {
    id: serial("id").primaryKey(),
    isVaalGem: boolean("is_vaal_gem"),
    baseTypeName: varchar("base_type_name").notNull(),
    explicitMods: jsonb("explicit_mods"),
    secDescrText: varchar("sec_descr_text"),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const extended = pgTable("extended", {
    id: serial("id").primaryKey(),
    // category: varchar("category"),
    // subcategories: jsonb("subcategories"),
    prefixes: integer("prefixes"),
    suffixes: integer("suffixes"),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const gemPages = pgTable("gem_pages", {
    id: serial("id").primaryKey(),
    skillName: varchar("skill_name"),
    description: varchar("description"),
    stats: jsonb("stats"),
    gemTabId: integer("gem_tab_id").references(() => gemTabs.id),
});

export const gemTabs = pgTable("gem_tabs", {
    id: serial("id").primaryKey(),
    name: varchar("name"),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const itemSockets = pgTable("item_sockets", {
    id: serial("id").primaryKey(),
    group: integer("group").notNull(),
    attr: itemSocketAttrEnum("attr"),
    sColour: itemSocketSColourEnum("s_colour"),
    type: itemSocketTypeEnum("type"),
    item: itemSocketItemEnum("item"),
    itemId: varchar("item_id")
        .notNull()
        .references(() => items.id),
});

export const leagues = pgTable("leagues", {
    id: varchar("id").primaryKey(),
    realm: varchar("realm"),
    description: text("description"),
    category_id: varchar("category_id"),
    category_current: boolean("category_current"),
    registerAt: varchar("register_at"),
    event: boolean("event"),
    url: varchar("url"),
    startAt: varchar("start_at"),
    endAt: varchar("end_at"),
    timedEvent: boolean("timed_event"),
    scoreEvent: boolean("score_event"),
    delveEvent: boolean("delve_event"),
    ancestorEvent: boolean("ancestor_event"),
    leagueEvent: boolean("league_event"),
});
