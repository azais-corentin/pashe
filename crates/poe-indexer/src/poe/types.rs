use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicStashTabs {
    pub next_change_id: String,
    pub stashes: Vec<PublicStashChange>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicStashChange {
    pub id: String,
    pub public: bool,
    pub account_name: Option<String>,
    pub stash: Option<String>,
    pub stash_type: String,
    pub league: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub realm: Option<String>,
    pub verified: bool,
    pub w: u32,
    pub h: u32,
    pub icon: String,
    pub support: Option<bool>,
    pub stack_size: Option<i32>,
    pub max_stack_size: Option<i32>,
    pub stack_size_text: Option<String>,
    pub league: Option<String>,
    pub id: Option<String>,
    pub influences: Option<Influences>,
    pub elder: Option<bool>,
    pub shaper: Option<bool>,
    pub abyss_jewel: Option<bool>,
    pub delve: Option<bool>,
    pub fractured: Option<bool>,
    pub synthesised: Option<bool>,
    pub sockets: Option<Vec<Socket>>,
    pub socketed_items: Option<Vec<Item>>,
    pub name: String,
    pub type_line: String,
    pub base_type: String,
    pub identified: bool,
    pub ilvl: i32,
    pub note: Option<String>,
    pub locked_to_character: Option<bool>,
    pub locked_to_account: Option<bool>,
    pub duplicated: Option<bool>,
    pub split: Option<bool>,
    pub corrupted: Option<bool>,
    pub unmodifiable: Option<bool>,
    pub cis_race_reward: Option<bool>,
    pub sea_race_reward: Option<bool>,
    pub th_race_reward: Option<bool>,
    pub properties: Option<Vec<Property>>,
    pub notable_properties: Option<Vec<Property>>,
    pub requirements: Option<Vec<Property>>,
    pub additional_properties: Option<Vec<Property>>,
    pub next_level_requirements: Option<Vec<Property>>,
    pub talisman_tier: Option<i32>,
    pub sec_descr_text: Option<String>,
    pub utility_mods: Option<Vec<String>>,
    pub logbook_mods: Option<Vec<LogbookMod>>,
    pub enchant_mods: Option<Vec<String>>,
    pub implicit_mods: Option<Vec<String>>,
    pub explicit_mods: Option<Vec<String>>,
    pub crafted_mods: Option<Vec<String>>,
    pub fractured_mods: Option<Vec<String>>,
    pub cosmetic_mods: Option<Vec<String>>,
    pub veiled_mods: Option<Vec<String>>,
    pub veiled: Option<bool>,
    pub descr_text: Option<String>,
    pub flavour_text: Option<Vec<String>>,
    pub prophecy_text: Option<String>,
    pub is_relic: Option<bool>,
    pub replica: Option<bool>,
    pub incubated_item: Option<IncubatedItem>,
    pub frame_type: Option<i32>,
    pub art_filename: Option<String>,
    pub hybrid: Option<Hybrid>,
    pub extended: Option<Extended>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub inventory_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Influences {
    pub elder: Option<bool>,
    pub shaper: Option<bool>,
    pub searing: Option<bool>,
    pub tangled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extended {
    pub prefixes: Option<u32>,
    pub suffixes: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Socket {
    pub group: i32,
    pub attr: Option<String>,
    #[serde(rename = "sColour")]
    pub s_colour: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    pub values: Vec<(String, i32)>,
    pub display_mode: i32,
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    pub progress: Option<f32>,
    pub suffix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hybrid {
    pub base_type_name: String,
    pub is_vaal_gem: Option<bool>,
    pub properties: Option<Vec<Property>>,
    pub sec_descr_text: Option<String>,
    pub explicit_mods: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncubatedItem {
    pub name: String,
    pub level: u32,
    pub progress: u32,
    pub total: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogbookMod {
    pub name: String,
    pub faction: Faction,
    pub mods: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub id: String,
    pub name: String,
}
