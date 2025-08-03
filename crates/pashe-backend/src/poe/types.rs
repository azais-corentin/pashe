use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicStashTabs {
    pub next_change_id: String,
    pub stashes: Vec<Stash>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stash {
    pub id: String,
    pub public: bool,
    pub account_name: Option<String>,
    pub stash: Option<String>,
    pub stash_type: String,     // LowCardinality
    pub league: Option<String>, // LowCardinality
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub realm: Option<String>, // LowCardinality
    pub verified: bool,
    pub w: i64,
    pub h: i64,
    pub icon: String, // Different icons for each variant (corrupted, influences, ...)
    pub support: Option<bool>,
    pub stack_size: Option<i64>,
    pub max_stack_size: Option<i64>,
    pub stack_size_text: Option<String>, // LowCardinality
    pub league: String,                  // LowCardinality
    pub id: String,
    pub gem_sockets: Option<Vec<String>>, // PoE2 only, LowCardinality
    pub influences: Option<Influences>,
    pub memory_item: Option<bool>,
    pub abyss_jewel: Option<bool>,
    pub delve: Option<bool>,
    pub fractured: Option<bool>,
    pub synthesised: Option<bool>,
    pub sockets: Option<Vec<Socket>>,
    pub socketed_items: Option<Vec<Item>>,
    pub name: String,
    pub type_line: String, // Includes prefixes and suffixes
    pub base_type: String,
    pub rarity: Option<Rarity>,
    pub identified: bool,
    pub item_level: Option<i64>,
    pub unidentifed_tier: Option<i64>, // PoE2 only
    pub ilvl: i64,
    pub note: Option<String>,
    // pub forum_note: Option<String>, // LowCardinality
    pub locked_to_character: Option<bool>,
    pub locked_to_account: Option<bool>,
    pub duplicated: Option<bool>,
    pub split: Option<bool>,
    pub corrupted: Option<bool>,
    pub unmodifiable: Option<bool>,
    pub unmodifiable_except_chaos: Option<bool>,
    //
    pub cis_race_reward: Option<bool>,
    pub sea_race_reward: Option<bool>,
    pub th_race_reward: Option<bool>,
    pub properties: Option<Vec<ItemProperty>>,
    pub notable_properties: Option<Vec<ItemProperty>>,
    pub requirements: Option<Vec<ItemProperty>>,
    pub weapon_requirements: Option<Vec<ItemProperty>>,
    pub support_gem_requirements: Option<Vec<ItemProperty>>,
    pub additional_properties: Option<Vec<ItemProperty>>,
    pub next_level_requirements: Option<Vec<ItemProperty>>,
    pub granted_skills: Option<Vec<ItemProperty>>,
    pub talisman_tier: Option<i64>,
    pub rewards: Option<Vec<Reward>>,
    pub sec_descr_text: Option<String>,
    pub utility_mods: Option<Vec<String>>, // LowCardinality
    pub logbook_mods: Option<Vec<LogbookMod>>,
    pub enchant_mods: Option<Vec<String>>,  // LowCardinality
    pub rune_mods: Option<Vec<String>>,     // PoE2 only, LowCardinality
    pub scourge_mods: Option<Vec<String>>,  // LowCardinality
    pub implicit_mods: Option<Vec<String>>, // LowCardinality
    pub ultimatum_mods: Option<Vec<UltimatumMod>>,
    pub explicit_mods: Option<Vec<String>>,  // LowCardinality
    pub crafted_mods: Option<Vec<String>>,   // LowCardinality
    pub fractured_mods: Option<Vec<String>>, // LowCardinality
    pub crucible_mods: Option<Vec<String>>,  // LowCardinality
    pub cosmetic_mods: Option<Vec<String>>,  // LowCardinality
    pub veiled_mods: Option<Vec<String>>,    // LowCardinality
    pub veiled: Option<bool>,
    // pub gem_tabs: Option<Vec<GemTab>>, // PoE2 only
    // pub gem_background: Option<String>, // PoE2 only
    // pub gem_skill: Option<String>,      // PoE2 only
    pub descr_text: Option<String>,
    pub flavour_text: Option<Vec<String>>, // LowCardinality
    // pub flavour_text_parsed: Option<Vec<String or Object>>,
    pub flavour_text_note: Option<String>, // LowCardinality
    pub prophecy_text: Option<String>,
    pub is_relic: Option<bool>,
    pub foil_variation: Option<i64>,
    pub replica: Option<bool>,
    pub foreseeing: Option<bool>,
    pub incubated_item: Option<IncubatedItem>,
    pub scourged: Option<ScourgedItem>,
    pub crucible: Option<Crucible>,
    pub ruthless: Option<bool>,
    pub frame_type: u8,
    pub art_filename: Option<String>, // LowCardinality
    pub hybrid: Option<Hybrid>,
    pub extended: Option<Extended>,
    pub x: Option<i64>,
    pub y: Option<i64>,
    pub inventory_id: Option<String>, // LowCardinality
    pub socket: Option<i64>,
    pub colour: Option<SocketAttribute>,
    //
    //
    //
    //
    //
    //
    //
    //
    //
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemProperty {
    pub name: String,
    pub values: Vec<Vec<Value>>,
    pub display_mode: i64,
    pub progress: Option<f64>,
    #[serde(rename = "type")]
    pub item_property_type: Option<i64>,
    pub suffix: Option<Suffix>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Suffix {
    Beast,
    Construct,
    Demon,
    Eldritch,
    #[serde(rename = "(gem)")]
    Gem,
    Humanoid,
    #[serde(rename = "(jewel)")]
    Jewel,
    Undead,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crucible {
    pub layout: String,
    pub nodes: HashMap<String, Node>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub orbit: i64,
    pub orbit_index: i64,
    pub icon: String,
    pub allocated: Option<bool>,
    pub stats: Vec<String>,
    #[serde(rename = "in")]
    pub node_in: Vec<String>,
    pub out: Vec<String>,
    pub skill: i64,
    pub tier: i64,
    pub is_notable: Option<bool>,
    pub is_reward: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extended {
    pub prefixes: Option<i64>,
    pub suffixes: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hybrid {
    pub is_vaal_gem: Option<bool>,
    pub base_type_name: String, // LowCardinality
    pub properties: Option<Vec<ItemProperty>>,
    pub explicit_mods: Option<Vec<String>>,
    pub sec_descr_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncubatedItem {
    pub name: Option<String>, // LowCardinality
    pub level: Option<i64>,
    pub progress: Option<i64>,
    pub total: Option<i64>,
    pub tier: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScourgedItem {
    pub name: Option<String>, // LowCardinality
    pub level: Option<i64>,
    pub progress: Option<i64>,
    pub total: Option<i64>,
    pub tier: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Influences {
    pub shaper: Option<bool>,
    pub elder: Option<bool>,
    pub hunter: Option<bool>,
    pub crusader: Option<bool>,
    pub redeemer: Option<bool>,
    pub warlord: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogbookMod {
    pub name: String, // LowCardinality
    pub faction: Faction,
    pub mods: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Faction {
    pub id: Id,
    pub name: String, // LowCardinality
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Id {
    Faction1,
    Faction2,
    Faction3,
    Faction4,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Rarity {
    Magic,
    Normal,
    Rare,
    Unique,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reward {
    pub label: String,
    pub rewards: HashMap<String, i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketAttribute {
    S,
    D,
    I,
    G,
    A,
    #[serde(rename = "DV")]
    Dv,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Socket {
    pub group: i64,
    pub attr: SocketAttribute,
    pub s_colour: SocketColour,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketColour {
    R,
    G,
    B,
    W,
    A,
    #[serde(rename = "DV")]
    Dv,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UltimatumMod {
    #[serde(rename = "type")]
    pub ultimatum_mod_type: String, // LowCardinality
    pub tier: i64,
}
