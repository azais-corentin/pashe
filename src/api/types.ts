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
    rewards: z.array(z.record(z.string(), z.number())),
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
    allocated: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isNotable: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isReward: z.boolean().nullish(),
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
    category: z.string().nullish(),
    subcategories: z.array(z.string()).nullish(),
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
    support: z.boolean().nullish(),
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
    elder: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    shaper: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    searing: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    tangled: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    memoryItem: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    abyssJewel: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    delve: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    fractured: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    synthesised: z.boolean().nullish(),
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
    lockedToCharacter: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    lockedToAccount: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    duplicated: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    split: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    corrupted: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    unmodifiable: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    cisRaceReward: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    seaRaceReward: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    thRaceReward: z.boolean().nullish(),
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
    veiled: z.boolean().nullish(),
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
    isRelic: z.boolean().nullish(),
    foilVariation: z.number().nullish(),
    /**
     * Always `true` if present.
     */
    replica: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    foreseeing: z.boolean().nullish(),
    incubatedItem: IncubatedItemSchema.nullish(),
    scourged: ScourgedSchema.nullish(),
    crucible: CrucibleSchema.nullish(),
    /**
     * Always `true` if present.
     */
    ruthless: z.boolean().nullish(),
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
    colour: z.string().nullish(),
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
            current: z.boolean().nullish(),
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
    event: z.boolean().nullish(),
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
    timedEvent: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    scoreEvent: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    delveEvent: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    ancestorEvent: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    leagueEvent: z.boolean().nullish(),
});
export type League = z.infer<typeof LeagueSchema>;

export const GuildSchema = z.object({
    id: z.number(),
    name: z.string(),
    tag: z.string(),
});
export type Guild = z.infer<typeof GuildSchema>;

export const AccountSchema = z.object({
    name: z.string(),
    /**
     * `pc`, `xbox`, or `sony`.
     */
    realm: z.string().nullish(),
    guild: GuildSchema.nullish(),
    challenges: z
        .object({
            /**
             * The challenge set.
             */
            set: z.string(),
            completed: z.number(),
            max: z.number(),
        })
        .nullish(),
    twitch: z
        .object({
            name: z.string(),
            stream: z
                .object({
                    name: z.string(),
                    image: z.string(),
                    status: z.string(),
                })
                .nullish(),
        })
        .nullish(),
});
export type Account = z.infer<typeof AccountSchema>;

export const LadderEntrySchema = z.object({
    rank: z.number(),
    dead: z.boolean().nullish(),
    retired: z.boolean().nullish(),
    ineligible: z.boolean().nullish(),
    public: z.boolean().nullish(),
    character: z.object({
        /**
         * A unique 64 digit hexadecimal string.
         */
        id: z.string(),
        name: z.string(),
        level: z.number(),
        class: z.string(),
        /**
         * Time taken to complete the league objective in seconds.
         */
        time: z.number().nullish(),
        /**
         * Count of league objective completions.
         */
        score: z.number().nullish(),
        /**
         * The values of this depend on the league objective.
         */
        progress: z.object({}).nullish(),
        experience: z.number().nullish(),
        /**
         * Deepest Delve depth completed.
         */
        depth: z
            .object({
                default: z.number().nullish(),
                solo: z.number().nullish(),
            })
            .nullish(),
    }),
    account: AccountSchema.nullish(),
});
export type LadderEntry = z.infer<typeof LadderEntrySchema>;

export const EventLadderEntrySchema = z.object({
    rank: z.number(),
    ineligible: z.boolean().nullish(),
    /**
     * Time taken to complete the league objective in seconds.
     */
    time: z.number().nullish(),
    private_league: z.object({
        name: z.string(),
        /**
         * A URL link to a Path of Exile Private League.
         */
        url: z.string(),
    }),
});
export type EventLadderEntry = z.infer<typeof EventLadderEntrySchema>;

export const PvPMatchSchema = z.object({
    /**
     * The match's name.
     */
    id: z.string(),
    /**
     * `pc`, `xbox`, or `sony`.
     */
    realm: z.string().nullish(),
    /**
     * Date time (ISO8601).
     */
    startAt: z.string().nullish(),
    /**
     * Date time (ISO8601).
     */
    endAt: z.string().nullish(),
    /**
     * A URL link to a Path of Exile forum thread.
     */
    url: z.string().nullish(),
    description: z.string(),
    glickoRatings: z.boolean(),
    /**
     * Always `true`.
     */
    pvp: z.boolean(),
    /**
     * `Blitz`, `Swiss`, or `Arena`.
     */
    style: z.string(),
    /**
     * Date time (ISO8601).
     */
    registerAt: z.string().nullish(),
    /**
     * Always `true` if present.
     */
    complete: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    upcoming: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    inProgress: z.boolean().nullish(),
});
export type PvPMatch = z.infer<typeof PvPMatchSchema>;

export const PvPLadderTeamMemberSchema = z.object({
    account: AccountSchema,
    character: z.object({
        /**
         * A unique 64 digit hexadecimal string.
         */
        id: z.string(),
        name: z.string(),
        level: z.number(),
        class: z.string(),
        league: z.string().nullish(),
        /**
         * Count of league objective completions.
         */
        score: z.number().nullish(),
    }),
    /**
     * Always `true` if present.
     */
    public: z.boolean().nullish(),
});
export type PvPLadderTeamMember = z.infer<typeof PvPLadderTeamMemberSchema>;

export const PvPLadderTeamEntrySchema = z.object({
    rank: z.number(),
    /**
     * Only present if the PvP Match uses Glicko ratings.
     */
    rating: z.number().nullish(),
    points: z.number().nullish(),
    games_played: z.number().nullish(),
    cumulative_opponent_points: z.number().nullish(),
    /**
     * Date time (ISO8601).
     */
    last_game_time: z.string().nullish(),
    members: z.array(PvPLadderTeamMemberSchema),
});
export type PvPLadderTeamEntry = z.infer<typeof PvPLadderTeamEntrySchema>;

export const PassiveGroupSchema = z.object({
    x: z.number(),
    y: z.number(),
    orbits: z.array(z.number()),
    /**
     * Always `true` if present.
     */
    isProxy: z.boolean().nullish(),
    /**
     * Identifier of the placeholder node.
     */
    proxy: z.string().nullish(),
    /**
     * The node identifiers associated with this group.
     */
    nodes: z.array(z.string()),
});
export type PassiveGroup = z.infer<typeof PassiveGroupSchema>;

export const PassiveNodeSchema = z.object({
    /**
     * Skill hash.
     */
    skill: z.number().nullish(),
    name: z.string().nullish(),
    icon: z.string().nullish(),
    /**
     * Always `true` if present.
     */
    isKeystone: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isNotable: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isMastery: z.boolean().nullish(),
    /**
     * Inactive mastery image.
     */
    inactiveIcon: z.string().nullish(),
    /**
     * Active mastery image.
     */
    activeIcon: z.string().nullish(),
    /**
     * Active mastery or tattoo background image.
     */
    activeEffectImage: z.string().nullish(),
    masteryEffects: z
        .array(
            z.object({
                /**
                 * Effect hash.
                 */
                effect: z.number(),
                /**
                 * Stat descriptions.
                 */
                stats: z.array(z.string()),
                reminderText: z.array(z.string()).nullish(),
            }),
        )
        .nullish(),
    /**
     * Always `true` if present.
     */
    isBlighted: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isTattoo: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isProxy: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isJewelSocket: z.boolean().nullish(),
    /**
     * Cluster jewel information.
     */
    expansionJewel: z
        .object({
            size: z.number().nullish(),
            index: z.number().nullish(),
            /**
             * The proxy node identifier.
             */
            proxy: z.number().nullish(),
            /**
             * The parent node identifier.
             */
            parent: z.number().nullish(),
        })
        .nullish(),
    /**
     * Components required for Blight crafting this node. Each string is one of `ClearOil`, `SepiaOil`, `AmberOil`, `VerdantOil`, `TealOil`, `AzureOil`, `IndigoOil`, `VioletOil`, `CrimsonOil`, `BlackOil`, `OpalescentOil`, `SilverOil`, `GoldenOil`, or `PrismaticOil`.
     */
    recipe: z.array(z.string()).nullish(),
    /**
     * Sum of stats on this node that grant strength.
     */
    grantedStrength: z.number().nullish(),
    /**
     * Sum of stats on this node that grant dexterity.
     */
    grantedDexterity: z.number().nullish(),
    /**
     * Sum of stats on this node that grant intelligence.
     */
    grantedIntelligence: z.number().nullish(),
    ascendancyName: z.string().nullish(),
    /**
     * Always `true` if present.
     */
    isAscendancyStart: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isMultipleChoice: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    isMultipleChoiceOption: z.boolean().nullish(),
    grantedPassivePoints: z.number().nullish(),
    /**
     * Stat descriptions.
     */
    stats: z.array(z.string()).nullish(),
    reminderText: z.array(z.string()).nullish(),
    flavourText: z.array(z.string()).nullish(),
    classStartIndex: z.number().nullish(),
    /**
     * The key value to look up in the groups table.
     */
    group: z.string().nullish(),
    /**
     * The orbit this node occupies within it's group.
     */
    orbit: z.number().nullish(),
    /**
     * The index of this node in the group's orbit.
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
export type PassiveNode = z.infer<typeof PassiveNodeSchema>;

export const ItemJewelDataSchema = z.object({
    type: z.string(),
    radius: z.number().nullish(),
    radiusMin: z.number().nullish(),
    radiusVisual: z.string().nullish(),
    /**
     * Only present on cluster jewels.
     */
    subgraph: z
        .object({
            /**
             * The key is the string value of the group id.
             */
            groups: z.record(z.string(), PassiveGroupSchema),
            /**
             * The key is the string value of the node identifier.
             */
            nodes: z.record(z.string(), PassiveNodeSchema),
        })
        .nullish(),
});
export type ItemJewelData = z.infer<typeof ItemJewelDataSchema>;

export const CharacterSchema = z.object({
    /**
     * A unique 64 digit hexadecimal string.
     */
    id: z.string(),
    name: z.string(),
    /**
     * `pc`, `xbox`, or `sony`.
     */
    realm: z.string(),
    class: z.string(),
    league: z.string().nullish(),
    level: z.number(),
    experience: z.number(),
    /**
     * PoE1 only; always `true` if present.
     */
    ruthless: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    expired: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    deleted: z.boolean().nullish(),
    /**
     * Always `true` if present.
     */
    current: z.boolean().nullish(),
    equipment: z.array(ItemSchema).nullish(),
    /**
     * PoE2 only.
     */
    skills: z.array(ItemSchema).nullish(),
    inventory: z.array(ItemSchema).nullish(),
    rucksack: z.array(ItemSchema).nullish(),
    jewels: z.array(ItemSchema).nullish(),
    passives: z
        .object({
            hashes: z.array(z.number()),
            /**
             * PoE1 only.
             */
            hashes_ex: z.array(z.number()).nullish(),
            /**
             * PoE1 only; the key is the string value of the mastery node skill hash and the value is the selected effect hash.
             */
            mastery_effects: z.record(z.string(), z.number()).nullish(),
            /**
             * PoE2 only; the keys are `set1`, `set2`, and `shapeshift`.
             */
            specialisations: z.record(z.string(), z.array(z.number())).nullish(),
            /**
             * The key is the string value of the node identifier being replaced.
             */
            skill_overrides: z.record(z.string(), PassiveNodeSchema).nullish(),
            /**
             * PoE1 only; one of `Kraityn`, `Alira`, `Oak`, or `Eramir`.
             */
            bandit_choice: z.string().nullish(),
            /**
             * PoE1 only; one of `TheBrineKing`, `Arakaali`, `Solaris`, or `Lunaris`.
             */
            pantheon_major: z.string().nullish(),
            /**
             * PoE1 only; one of `Abberath`, `Gruthkul`, `Yugul`, `Shakari`, `Tukohama`, `Ralakesh`, `Garukhan`, or `Ryslatha`.
             */
            pantheon_minor: z.string().nullish(),
            /**
             * The key is the string value of the x property of an item from the jewels array in this request.
             */
            jewel_data: z.record(z.string(), ItemJewelDataSchema).nullish(),
            /**
             * @deprecated PoE1 only; `Warden`, `Warlock`, or `Primalist`.
             */
            alternate_ascendancy: z.string().nullish(),
        })
        .nullish(),
    metadata: z
        .object({
            /**
             * Game version for the character's realm.
             */
            version: z.string().nullish(),
        })
        .nullish(),
});
export type Character = z.infer<typeof CharacterSchema>;

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
        public: z.boolean().nullish(),
        /**
         * Always `true` if present.
         */
        folder: z.boolean().nullish(),
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
    public: z.boolean().nullish(),
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
