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

export interface PublicStashStream {
    next_change_id: string;
    stashes: PublicStashChange[];
}

interface PublicStashChange {
    id: string;
    public: boolean;
    accountName?: string;
    stash?: string;
    lastCharacterName?: string;
    stashType: string;
    league?: string;
    items: Item[];
}

interface Item {
    verified: boolean;
    w: number;
    h: number;
    icon: string;
    support?: boolean;
    stackSize?: number;
    maxStackSize?: number;
    stackSizeText?: string;
    league?: string;
    id?: string;
    influences?: object;
    elder?: boolean;
    shaper?: boolean;
    searing?: boolean;
    tangled?: boolean;
    abyssJewel?: boolean;
    delve?: boolean;
    fractured?: boolean;
    synthesised?: boolean;
    sockets?: ItemSocket[];
    socketedItems?: Item[];
    name: string;
    typeLine: string;
    baseType: string;
    identified: boolean;
    itemLevel?: number;
    ilvl: number;
    note?: string;
    forum_note?: string;
    lockedToCharacter?: boolean;
    lockedToAccount?: boolean;
    duplicated?: boolean;
    split?: boolean;
    corrupted?: boolean;
    unmodifiable?: boolean;
    cisRaceReward?: boolean;
    seaRaceReward?: boolean;
    thRaceReward?: boolean;
    properties?: ItemProperty[];
    notableProperties?: ItemProperty[];
    requirements?: ItemProperty[];
    additionalProperties?: ItemProperty[];
    nextLevelRequirements?: ItemProperty[];
    talismanTier?: number;
    rewards?: Reward[];
    secDescrText?: string;
    utilityMods?: string[];
    logbookMods?: LogbookMod[];
    enchantMods?: string[];
    scourgeMods?: string[];
    implicitMods?: string[];
    ultimatumMods?: UltimatumMod[];
    explicitMods?: string[];
    craftedMods?: string[];
    fracturedMods?: string[];
    crucibleMods?: string[];
    cosmeticMods?: string[];
    veiledMods?: string[];
    veiled?: boolean;
    descrText?: string;
    flavourText?: string[];
    flavourTextParsed?: string[] | JSON;
    flavourTextNote?: string;
    prophecyText?: string;
    isRelic?: boolean;
    foilVariation?: number;
    replica?: boolean;
    foreseeing?: boolean;
    incubatedItem?: IncubatedItem;
    scourged?: Scourged;
    crucible?: Crucible;
    ruthless?: boolean;
    frameType?: FrameType;
    artFilename?: string;
    hybrid?: Hybrid;
    extended?: Extended;
    x?: number;
    y?: number;
    inventoryId?: string;
    socket?: number;
    colour?: string;
}

type ItemSocket = {};

type ItemProperty = {};

interface Reward {
    label: string;
    rewards: { [key: string]: number }[];
}

interface LogbookMod {
    name: string;
    faction: { id: string; name: string };
    mods: string[];
}

interface UltimatumMod {
    type: string;
    tier: number;
}

interface IncubatedItem {
    name: string;
    level: number;
    progress: number;
    total: number;
}

interface Scourged {
    tier: number;
    level?: number;
    progress?: number;
    total?: number;
}

interface Crucible {
    layout: string;
    nodes: { [key: string]: CrucibleNode }[];
}

interface CrucibleNode {
    skill?: number;
    tier?: number;
    icon?: string;
    allocated?: boolean;
    isNotable?: boolean;
    isReward?: boolean;
    stats?: string[];
    reminderText?: string[];
    orbit?: number;
    orbitIndex?: number;
    out: string[];
    in: string[];
}

interface Hybrid {
    VaalGem?: boolean;
    TypeName: string;
    properties?: ItemProperty[];
    explicitMods?: string[];
    secDescrText?: string;
}

interface Extended {
    category?: string;
    subcategories?: string[];
    prefixes?: number;
    suffixes: number;
}

enum FrameType {
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
}
