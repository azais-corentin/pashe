import { z } from "zod";
import packageData from "../../package.json" with { type: "json" };

export const constants = {
    server: "https://api.pathofexile.com",
    defaultOptions: {
        headers: {
            "User-Agent": `OAuth ${packageData.name}/${packageData.version} (contact: ${packageData.author.email})`,
        },
    },
};

export const buildUrl = (endpoint: string) => {
    return `${constants.server}/${endpoint}`;
};

/**
 * Referenced by `ItemProperty.displayMode`.
 */
export enum DisplayMode {
    /**
     * Name should be followed by values.
     */
    NameFollowedByValues = 0,
    /**
     * Values should be followed by name.
     */
    ValuesFollowedByName = 1,
    /**
     * Progress bar.
     */
    ProgressBar = 2,
    /**
     * Values should be inserted into the string by index.
     */
    ValuesInsertedIntoStringByIndex = 3,
    /**
     * Separator.
     */
    Separator = 4,
}

/**
 * Referenced by `Item.frameType`.
 */
export enum FrameType {
    NormalFrame = 0,
    MagicFrame,
    RareFrame,
    UniqueFrame,
    GemFrame,
    CurrencyFrame,
    DivinationCardFrame,
    QuestFrame,
    ProphecyFrameLegacy,
    FoilFrame,
    SupporterFoilFrame,
    NecropolisFrame,
}

export const ItemPropertySchema = z.object({
    name: z.string(),
    values: z.array(z.tuple([z.string(), z.number()])),
    displayMode: z.enum(DisplayMode).nullish(),
    /**
     * Rounded to 2 decimal places.
     */
    progress: z.number().nullish(),
    type: z.number().nullish(),
    suffix: z.string().nullish(),
    icon: z.string().nullish(),
});
export type ItemProperty = z.infer<typeof ItemPropertySchema>;

export const RewardSchema = z.object({
    label: z.string(),
    /**
     * The key is a string representing the type of reward. The value is the amount.
     */
    rewards: z.record(z.string(), z.number()),
});
export type Reward = z.infer<typeof RewardSchema>;

export const LogbookModSchema = z.object({
    /**
     * Area name.
     */
    name: z.string(),
    faction: z.object({
        /**
         * `Faction1`, `Faction2`, `Faction3`, or `Faction4`.
         */
        id: z.string(),
        name: z.string(),
    }),
    mods: z.array(z.string()),
});
export type LogbookMod = z.infer<typeof LogbookModSchema>;

export const UltimatumModSchema = z.object({
    /**
     * Text used to display ultimatum icons.
     */
    type: z.string(),
    tier: z.number(),
});
export type UltimatumMod = z.infer<typeof UltimatumModSchema>;

export const IncubatedItemSchema = z.object({
    name: z.string(),
    /**
     * Monster level required to progress.
     */
    level: z.number(),
    progress: z.number(),
    total: z.number(),
});
export type IncubatedItem = z.infer<typeof IncubatedItemSchema>;

export const ScourgedSchema = z.object({
    /**
     * 1-3 for items, 1-10 for maps.
     */
    tier: z.number(),
    /**
     * Monster level required to progress.
     */
    level: z.number().nullish(),
    progress: z.number().nullish(),
    total: z.number().nullish(),
});
export type Scourged = z.infer<typeof ScourgedSchema>;

export const CrucibleNodeSchema = z.object({
    /**
     * Mod hash.
     */
    skill: z.number().nullish(),
    /**
     * Mod tier.
     */
    tier: z.number().nullish(),
    icon: z.string().nullish(),
    /**
     * Always `true` if present.
     */
    allocated: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    isNotable: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    isReward: z.boolean().default(false),
    /**
     * Stat descriptions.
     */
    stats: z.array(z.string()).nullish(),
    reminderText: z.array(z.string()).nullish(),
    /**
     * The column this node occupies.
     */
    orbit: z.number().nullish(),
    /**
     * The node's position within the column.
     */
    orbitIndex: z.number().nullish(),
    /**
     * Node identifiers of nodes this one connects to.
     */
    out: z.array(z.string()),
    /**
     * Node identifiers of nodes connected to this one.
     */
    in: z.array(z.string()),
});
export type CrucibleNode = z.infer<typeof CrucibleNodeSchema>;

export const CrucibleSchema = z.object({
    /**
     * URL to an image of the tree layout.
     */
    layout: z.string(),
    /**
     * The key is the string value of the node index.
     */
    nodes: z.record(z.string(), CrucibleNodeSchema),
});
export type Crucible = z.infer<typeof CrucibleSchema>;

export const HybridSchema = z.object({
    isVaalGem: z.boolean().nullish(),
    baseTypeName: z.string(),
    properties: z.array(ItemPropertySchema).nullish(),
    explicitMods: z.array(z.string()).nullish(),
    secDescrText: z.string().nullish(),
});
export type Hybrid = z.infer<typeof HybridSchema>;

export const ExtendedSchema = z.object({
    // category: z.string().nullish(),
    // subcategories: z.array(z.string()).nullish(),
    prefixes: z.number().nullish(),
    suffixes: z.number().nullish(),
});
export type Extended = z.infer<typeof ExtendedSchema>;

export const GemPageSchema = z.object({
    skillName: z.string().nullish(),
    description: z.string().nullish(),
    properties: z.array(ItemPropertySchema).nullish(),
    stats: z.array(z.string()).nullish(),
});
export type GemPage = z.infer<typeof GemPageSchema>;

export const GemTabSchema = z.object({
    name: z.string().nullish(),
    pages: z.array(GemPageSchema),
});
export type GemTab = z.infer<typeof GemTabSchema>;

export const ItemSocketSchema = z.object({
    group: z.number(),
    /**
     * PoE1 only; `S`, `D`, `I`, `G`, `A`, or `DV`.
     */
    attr: z.enum(["S", "D", "I", "G", "A", "DV"]).nullish(),
    /**
     * PoE1 only; `R`, `G`, `B`, `W`, `A`, or `DV`.
     */
    sColour: z.enum(["R", "G", "B", "W", "A", "DV"]).nullish(),
    /**
     * PoE2 only; `gem`, `jewel`, or `rune`.
     */
    type: z.enum(["gem", "jewel", "rune"]).nullish(),
    /**
     * PoE2 only.
     */
    item: z
        .enum([
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
        ])
        .nullish(),
});
export type ItemSocket = z.infer<typeof ItemSocketSchema>;

const ItemWithoutSocketedItemsSchema = z.object({
    /**
     * Always `poe2` if present.
     */
    realm: z.string().nullish(),
    verified: z.boolean(),
    w: z.number(),
    h: z.number(),
    icon: z.string(),
    /**
     * Always `true` if present.
     */
    support: z.boolean().default(false),
    stackSize: z.number().nullish(),
    maxStackSize: z.number().nullish(),
    stackSizeText: z.string().nullish(),
    league: z.string().nullish(),
    /**
     * A unique 64 digit hexadecimal string.
     */
    id: z.string().nullish(),
    /**
     * PoE2 only; string is always `W`.
     */
    gemSockets: z.array(z.string()).nullish(),
    influences: z.object({}).nullish(),
    /**
     * Always `true` if present.
     */
    elder: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    shaper: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    searing: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    tangled: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    memoryItem: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    abyssJewel: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    delve: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    fractured: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    synthesised: z.boolean().default(false),
    sockets: z.array(ItemSocketSchema).nullish(),
    name: z.string(),
    typeLine: z.string(),
    baseType: z.string(),
    rarity: z.enum(["Normal", "Magic", "Rare", "Unique"]).nullish(),
    identified: z.boolean(),
    /**
     * Used for items that always display their item level.
     */
    itemLevel: z.number().nullish(),
    /**
     * PoE2 only.
     */
    unidentifiedTier: z.number().nullish(),
    ilvl: z.number(),
    /**
     * User-generated text.
     */
    note: z.string().nullish(),
    /**
     * User-generated text.
     */
    forum_note: z.string().nullish(),
    /**
     * Always `true` if present.
     */
    lockedToCharacter: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    lockedToAccount: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    duplicated: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    split: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    corrupted: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    unmodifiable: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    cisRaceReward: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    seaRaceReward: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    thRaceReward: z.boolean().default(false),
    properties: z.array(ItemPropertySchema).nullish(),
    notableProperties: z.array(ItemPropertySchema).nullish(),
    requirements: z.array(ItemPropertySchema).nullish(),
    /**
     * PoE2 only.
     */
    weaponRequirements: z.array(ItemPropertySchema).nullish(),
    /**
     * PoE2 only.
     */
    supportGemRequirements: z.array(ItemPropertySchema).nullish(),
    additionalProperties: z.array(ItemPropertySchema).nullish(),
    nextLevelRequirements: z.array(ItemPropertySchema).nullish(),
    /**
     * PoE2 only.
     */
    grantedSkills: z.array(ItemPropertySchema).nullish(),
    talismanTier: z.number().nullish(),
    rewards: z.array(RewardSchema).nullish(),
    secDescrText: z.string().nullish(),
    utilityMods: z.array(z.string()).nullish(),
    logbookMods: z.array(LogbookModSchema).nullish(),
    enchantMods: z.array(z.string()).nullish(),
    /**
     * PoE2 only.
     */
    runeMods: z.array(z.string()).nullish(),
    scourgeMods: z.array(z.string()).nullish(),
    implicitMods: z.array(z.string()).nullish(),
    ultimatumMods: z.array(UltimatumModSchema).nullish(),
    explicitMods: z.array(z.string()).nullish(),
    craftedMods: z.array(z.string()).nullish(),
    fracturedMods: z.array(z.string()).nullish(),
    /**
     * Only allocated mods are included.
     */
    crucibleMods: z.array(z.string()).nullish(),
    cosmeticMods: z.array(z.string()).nullish(),
    /**
     * Random video identifier.
     */
    veiledMods: z.array(z.string()).nullish(),
    /**
     * Always `true` if present.
     */
    veiled: z.boolean().default(false),
    /**
     * PoE2 only.
     */
    gemTabs: z.array(GemTabSchema).nullish(),
    /**
     * PoE2 only.
     */
    gemBackground: z.string().nullish(),
    /**
     * PoE2 only.
     */
    gemSkill: z.string().nullish(),
    descrText: z.string().nullish(),
    flavourText: z.array(z.string()).nullish(),
    flavourTextParsed: z.array(z.union([z.string(), z.object({})])).nullish(),
    /**
     * User-generated text.
     */
    flavourTextNote: z.string().nullish(),
    prophecyText: z.string().nullish(),
    /**
     * Always `true` if present.
     */
    isRelic: z.boolean().default(false),
    foilVariation: z.number().nullish(),
    /**
     * Always `true` if present.
     */
    replica: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    foreseeing: z.boolean().default(false),
    incubatedItem: IncubatedItemSchema.nullish(),
    scourged: ScourgedSchema.nullish(),
    crucible: CrucibleSchema.nullish(),
    /**
     * Always `true` if present.
     */
    ruthless: z.boolean().default(false),
    frameType: z.enum(FrameType).nullish(),
    artFilename: z.string().nullish(),
    hybrid: HybridSchema.nullish(),
    /**
     * Only present in the Public Stash API.
     */
    extended: ExtendedSchema.nullish(),
    x: z.number().nullish(),
    y: z.number().nullish(),
    inventoryId: z.string().nullish(),
    socket: z.number().nullish(),
    /**
     * `S`, `D`, `I`, or `G`.
     */
    colour: z.enum(["S", "D", "I", "G"]).nullish(),
});
export const ItemSchema = ItemWithoutSocketedItemsSchema.extend({
    socketedItems: z.array(ItemWithoutSocketedItemsSchema).nullish(),
});

export type Item = z.infer<typeof ItemSchema>;

export const PublicStashChangeSchema = z.object({
    /**
     * A unique 64 digit hexadecimal string.
     */
    id: z.string(),
    /**
     * If `false` then optional properties will be `null`.
     */
    public: z.boolean(),
    /**
     * The account name.
     */
    accountName: z.string().nullish(),
    /**
     * The name of the stash.
     */
    stash: z.string().nullish(),
    /**
     * The type of the stash.
     */
    stashType: z.string(),
    /**
     * The league's name.
     */
    league: z.string().nullish(),
    /**
     * The items in the stash.
     */
    items: z.array(ItemSchema),
});
export type PublicStashChange = z.infer<typeof PublicStashChangeSchema>;

export const PublicStashStreamSchema = z.object({
    /**
     * A pagination code for the next request.
     */
    next_change_id: z.string(),
    /**
     * A list of public stash changes.
     */
    stashes: z.array(PublicStashChangeSchema),
});
export type PublicStashStream = z.infer<typeof PublicStashStreamSchema>;

export const LeagueRuleSchema = z.object({
    /**
     * Examples: `Hardcore`, `NoParties` (SSF).
     */
    id: z.string(),
    name: z.string(),
    description: z.string().nullish(),
});
export type LeagueRule = z.infer<typeof LeagueRuleSchema>;

export const LeagueSchema = z.object({
    /**
     * The league's name.
     */
    id: z.string(),
    /**
     * `pc`, `xbox`, or `sony`.
     */
    realm: z.string().nullish(),
    description: z.string().nullish(),
    category: z
        .object({
            /**
             * The league category, e.g. Affliction.
             */
            id: z.string(),
            /**
             * Set for the active challenge leagues; always `true` if present.
             */
            current: z.boolean().default(false),
        })
        .nullish(),
    rules: z.array(LeagueRuleSchema).nullish(),
    /**
     * Date time (ISO8601).
     */
    registerAt: z.string().nullish(),
    /**
     * Always `true` if present.
     */
    event: z.boolean().default(false),
    /**
     * A URL link to a Path of Exile forum thread.
     */
    url: z.string().nullish(),
    /**
     * Date time (ISO8601).
     */
    startAt: z.string().nullish(),
    /**
     * Date time (ISO8601).
     */
    endAt: z.string().nullish(),
    /**
     * Always `true` if present.
     */
    timedEvent: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    scoreEvent: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    delveEvent: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    ancestorEvent: z.boolean().default(false),
    /**
     * Always `true` if present.
     */
    leagueEvent: z.boolean().default(false),
});
export type League = z.infer<typeof LeagueSchema>;

export const GuildSchema = z.object({
    id: z.number(),
    name: z.string(),
    tag: z.string(),
});
export type Guild = z.infer<typeof GuildSchema>;

const StashTabWithoutChildrenSchema = z.object({
    /**
     * A 10 digit hexadecimal string.
     */
    id: z.string(),
    /**
     * A 10 digit hexadecimal string.
     */
    parent: z.string().nullish(),
    name: z.string(),
    type: z.string(),
    index: z.number().nullish(),
    metadata: z.object({
        /**
         * Always `true` if present.
         */
        public: z.boolean().default(false),
        /**
         * Always `true` if present.
         */
        folder: z.boolean().default(false),
        /**
         * 6 digit hex colour.
         */
        colour: z.string().nullish(),
    }),
    items: z.array(ItemSchema).nullish(),
});
export const StashTabSchema = StashTabWithoutChildrenSchema.extend({
    children: z.array(StashTabWithoutChildrenSchema).nullish(),
});
export type StashTab = z.infer<typeof StashTabSchema>;

export const LeagueAccountSchema = z.object({
    /**
     * @deprecated
     */
    atlas_passives: z
        .object({
            hashes: z.array(z.number()),
        })
        .nullish(),
    atlas_passive_trees: z.array(
        z.object({
            name: z.string(),
            hashes: z.array(z.number()),
        }),
    ),
});
export type LeagueAccount = z.infer<typeof LeagueAccountSchema>;

export const ItemFilterSchema = z.object({
    id: z.string(),
    filter_name: z.string(),
    realm: z.string(),
    description: z.string(),
    version: z.string(),
    /**
     * Either `Normal` or `Ruthless`.
     */
    type: z.string(),
    /**
     * Always `true` if present.
     */
    public: z.boolean().default(false),
    /**
     * Not present when listing all filters.
     */
    filter: z.string().nullish(),
    /**
     * Not present when listing all filters.
     */
    validation: z
        .object({
            valid: z.boolean(),
            /**
             * Game version.
             */
            version: z.string().nullish(),
            /**
             * Date time (ISO8601).
             */
            validated: z.string().nullish(),
        })
        .nullish(),
});
export type ItemFilter = z.infer<typeof ItemFilterSchema>;
