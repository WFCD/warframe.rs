use std::{
    collections::HashMap,
    fmt,
};

use serde::{
    de::{
        self,
        Visitor,
    },
    Deserialize,
    Deserializer,
    Serialize,
};
use serde_json::Value;
use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ProfilePayload {
    pub(crate) results: Vec<Profile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    #[serde(deserialize_with = "deserialize_oid")]
    /// account_id
    pub account_id: String,

    /// display_name
    pub display_name: PlatformName,

    /// platform_names
    pub platform_names: Option<Vec<PlatformName>>,

    /// player_level
    pub player_level: u8,

    /// load_out_preset
    pub load_out_preset: Option<LoadOutPreset>,

    /// load_out_inventory
    pub load_out_inventory: LoadOutInventory,

    #[serde(deserialize_with = "deserialize_oid_or_none")]
    /// guild_id
    pub guild_id: Option<String>,

    /// guild_name
    pub guild_name: Option<String>,

    /// guild_tier
    pub guild_tier: Option<GuildTier>,

    #[serde(rename = "GuildXP")]
    /// guild_xp
    pub guild_xp: Option<i32>,

    /// guild_class
    pub guild_class: Option<u8>,

    /// guild_emblem
    pub guild_emblem: Option<bool>,

    #[serde(deserialize_with = "deserialize_oid_or_none")]
    /// alliance_id
    pub alliance_id: Option<String>,

    /// player_skills
    pub player_skills: HashMap<PlayerSkill, i32>,

    /// challenge_progress
    pub challenge_progress: Vec<ChallengeProgress>,

    /// death_marks
    pub death_marks: Vec<String>,

    /// harvestable
    pub harvestable: bool,

    /// death_squadable
    pub death_squadable: bool,

    #[serde(deserialize_with = "deserialize_date")]
    /// created
    pub created: i64,

    /// migrated_to_console
    pub migrated_to_console: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct PlatformName {
    /// name
    pub name: String,

    /// platform
    pub platform: Platform,
}

impl<'de> Deserialize<'de> for PlatformName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitFieldVisitor;

        impl<'de> Visitor<'de> for SplitFieldVisitor {
            type Value = PlatformName;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with a byte code for platform at the end")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Split string
                let (display_name, platform_byte) = value.split_at(value.len() - 1);
                // Convert last char of string to Platform enum
                let platform = Platform::from_byte(platform_byte.as_bytes()[0])
                    .ok_or_else(|| E::custom("invalid platform byte"))?;
                Ok(PlatformName {
                    name: display_name.to_string(),
                    platform,
                })
            }
        }

        deserializer.deserialize_str(SplitFieldVisitor)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Platform {
    PC = 0x0,
    Xbox = 0x1,
    PS = 0x2,
    Switch = 0x3,
    Ios = 0x4,
}

impl Platform {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x0 => Some(Self::PC),
            0x1 => Some(Self::Xbox),
            0x2 => Some(Self::PS),
            0x3 => Some(Self::Switch),
            0x4 => Some(Self::Ios),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutPreset {
    /// focus_school
    pub focus_school: FocusSchool,

    /// preset_icon
    pub preset_icon: String,

    /// favorite
    pub favorite: bool,

    #[serde(rename = "n")]
    /// name, only present if set
    pub name: Option<String>,

    #[serde(rename = "s")]
    /// warframe
    pub warframe: LoadOutPresetItem,

    #[serde(rename = "l")]
    /// primary
    pub primary: LoadOutPresetItem,

    #[serde(rename = "p")]
    /// secondary
    pub secondary: LoadOutPresetItem,

    #[serde(rename = "m")]
    /// melee
    pub melee: LoadOutPresetItem,

    #[serde(rename = "h")]
    // TODO: What is this?
    /// h
    pub h: LoadOutPresetItem,

    #[serde(rename = "a")]
    // TODO: What is this?
    /// a
    pub a: Option<LoadOutPresetItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutPresetItem {
    #[serde(deserialize_with = "deserialize_oid")]
    /// item_id
    pub item_id: String,

    #[serde(rename = "mod")]
    /// mod_loadout
    pub mod_loadout: u8,

    #[serde(rename = "cus")]
    /// customisation_loadout
    pub customisation_loadout: u8,

    #[serde(rename = "hide", default = "bool::default")]
    /// hide, only present in API if true
    pub hide: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FocusSchool {
    #[serde(rename = "AP_ATTACK")]
    Madurai,
    #[serde(rename = "AP_DEFENSE")]
    Vazarin,
    #[serde(rename = "AP_TACTIC")]
    Naramon,
    #[serde(rename = "AP_WARD")]
    Unairu,
    #[serde(rename = "AP_POWER")]
    Zenurik,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventory {
    /// weapon_skins
    pub weapon_skins: Vec<LoadOutInventoryItemType>,

    #[serde(rename = "Suits")]
    /// warframes
    pub warframe: Vec<LoadOutInventoryItem<WarframeLoadOutInventoryItemConfig>>,

    #[serde(rename = "LongGuns")]
    /// primaries
    pub primaries: Vec<LoadOutInventoryItem>,

    #[serde(rename = "Pistols")]
    /// secondaries
    pub secondaries: Vec<LoadOutInventoryItem>,

    /// melee
    pub melee: Vec<LoadOutInventoryItem>,

    #[serde(rename = "XPInfo")]
    /// xp_info
    pub xp_info: Vec<XPInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItemType {
    /// item_type
    pub item_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItem<Config = LoadOutInventoryItemConfig> {
    /// item_type
    pub item_type: String,

    /// configs
    pub configs: Vec<Config>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LoadOutInventoryItemConfig {
    /// name
    pub name: String,

    /// skins
    pub skins: Vec<String>,

    #[serde(rename = "pricol")]
    /// primary_colors
    pub primary_colors: ColorLoadOut,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WarframeLoadOutInventoryItemConfig {
    #[serde(flatten)]
    pub base: LoadOutInventoryItemConfig,

    #[serde(rename = "attcol")]
    /// attachment_colors
    pub attachment_colors: ColorLoadOut,

    #[serde(rename = "sigcol")]
    /// sigil_colors
    pub sigil_colors: SigilColorLoadOut,

    #[serde(rename = "syancol")]
    /// syandana_colors
    pub syandana_colors: ColorLoadOut,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// base10 i32 color loadout, [None] if color is not set
pub struct ColorLoadOut {
    #[serde(rename = "t0")]
    /// primary
    pub primary: Option<i32>,

    #[serde(rename = "t1")]
    /// secondary
    pub secondary: Option<i32>,

    #[serde(rename = "t2")]
    /// tertiary
    pub tertiary: Option<i32>,

    #[serde(rename = "t3")]
    /// accents
    pub accents: Option<i32>,

    #[serde(rename = "m0")]
    /// emissive_primary
    pub emissive_primary: Option<i32>,

    #[serde(rename = "m1")]
    /// emissive_secondary
    pub emissive_secondary: Option<i32>,

    #[serde(rename = "en")]
    /// energy_primary
    pub energy_primary: Option<i32>,

    #[serde(rename = "e1")]
    /// energy_secondary
    pub energy_secondary: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// base10 i32 color loadout, [None] if color is not set
pub struct SigilColorLoadOut {
    #[serde(rename = "t0")]
    /// front_primary
    pub front_primary: Option<i32>,

    #[serde(rename = "m0")]
    /// front_secondary
    pub front_secondary: Option<i32>,

    #[serde(rename = "t2")]
    /// back_primary
    pub back_primary: Option<i32>,

    #[serde(rename = "m1")]
    /// back_secondary
    pub back_secondary: Option<i32>,

    // TODO: What is this?
    /// t1
    pub t1: Option<i32>,

    // TODO: What is this?
    /// t3
    pub t3: Option<i32>,

    // TODO: What is this?
    /// en
    pub en: Option<i32>,

    // TODO: What is this?
    /// e1
    pub e1: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct XPInfo {
    /// item_type
    pub item_type: String,

    #[serde(rename = "XP")]
    /// xp
    pub xp: i32,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum GuildTier {
    Ghost = 1,
    Shadow = 2,
    Storm = 3,
    Mountain = 4,
    Moon = 5,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerSkill {
    #[serde(rename = "LPP_SPACE")]
    Railjack,
    #[serde(rename = "LPS_GUNNERY")]
    RailjackGunnery,
    #[serde(rename = "LPS_TACTICAL")]
    RailjackTactical,
    #[serde(rename = "LPS_PILOTING")]
    RailjackPiloting,
    #[serde(rename = "LPS_ENGINEERING")]
    RailjackEngineering,
    #[serde(rename = "LPS_COMMAND")]
    RailjackCommand,
    #[serde(rename = "LPP_DRIFTER")]
    Drifter,
    #[serde(rename = "LPS_DRIFT_RIDING")]
    DrifterRiding,
    #[serde(rename = "LPS_DRIFT_COMBAT")]
    DrifterCombat,
    #[serde(rename = "LPS_DRIFT_OPPORTUNITY")]
    DrifterOpportunity,
    #[serde(rename = "LPS_DRIFT_ENDURANCE")]
    DrifterEndurance,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChallengeProgress {
    // TODO: Do we want this as a BIG enum? Prob not
    /// name
    pub name: String,

    /// progress
    pub progress: i32,
}

fn deserialize_oid<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_oid_or_none(deserializer)?.ok_or_else(|| de::Error::custom("missing $oid field"))
}

fn deserialize_oid_or_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    if let Some(oid) = v.get("$oid").and_then(Value::as_str) {
        Ok(Some(oid.to_string()))
    } else {
        Ok(None)
    }
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    if let Some(date) = v.get("$date").and_then(Value::as_object) {
        if let Some(numberLong) = date.get("$numberLong").and_then(Value::as_str) {
            if let Ok(number) = numberLong.parse::<i64>() {
                Ok(number)
            } else {
                Err(de::Error::custom("invalid $numberLong field"))
            }
        } else {
            Err(de::Error::custom("missing $numberLong field"))
        }
    } else {
        Err(de::Error::custom("missing $date field"))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_profile_payload_deserialization() {
        let data = json!({
            "Results": [
                {
                    "AccountId": { "$oid": "507f1f77bcf86cd799439011" },
                    "DisplayName": "Player1\x00",
                    "PlatformNames": [
                        "Player1\x00",
                        "Player1\x01",
                        "Player1\x02",
                        "Player1\x03",
                        "Player1\x04",
                    ],
                    "PlayerLevel": 0,
                    "GuildId": { "$oid": "507f1f77bcf86cd799439011" },
                    "LoadOutPreset": {
                        "FocusSchool": "AP_DEFENSE",
                        "PresetIcon": "",
                        "Favorite": false,
                        "n": "Preset1",
                        "s": {
                            "ItemId": { "$oid": "507f1f77bcf86cd799439011" },
                            "mod": 0,
                            "cus": 0
                        },
                        "p": {
                            "ItemId": { "$oid": "507f1f77bcf86cd799439011" },
                            "mod": 0,
                            "cus": 0,
                            "hide": false
                        },
                        "l": {
                            "ItemId": { "$oid": "507f1f77bcf86cd799439011" },
                            "mod": 0,
                            "cus": 0,
                        },
                        "m": {
                            "ItemId": { "$oid": "507f1f77bcf86cd799439011" },
                            "mod": 0,
                            "cus": 0
                        },
                        "h": {
                            "ItemId": { "$oid": "507f1f77bcf86cd799439011" },
                            "mod": 0,
                            "cus": 0
                        }
                    },
                    "LoadOutInventory": {
                        "WeaponSkins": [
                            {
                                "ItemType": "Item1"
                            }
                        ],
                        "Suits": [
                            {
                                "ItemType": "Warframe",
                                "Configs": [
                                    {
                                        "Name": "Build1",
                                        "Skins": [],
                                        "pricol": {
                                            "t0": 0,
                                            "t1": 0,
                                            "t2": 0,
                                            "t3": 0,
                                            "m0": 0,
                                            "en": 0,
                                            "e1": 0
                                        },
                                        "attcol": {
                                            "t0": 0,
                                            "t1": 0,
                                            "t2": 0,
                                            "t3": 0,
                                            "m0": 0,
                                            "m1": 0,
                                            "en": 0,
                                            "e1": 0
                                        },
                                        "sigcol": {
                                            "t0": 0,
                                            "t1": 0,
                                            "t2": 0,
                                            "t3": 0,
                                            "m0": 0,
                                            "m1": 0,
                                            "en": 0,
                                            "e1": 0
                                        },
                                        "syancol": {
                                            "t0": 0,
                                            "t1": 0,
                                            "t2": 0,
                                            "t3": 0,
                                            "en": 0,
                                        }
                                    }
                                ]
                            }
                        ],
                        "LongGuns": [
                            {
                                "ItemType": "Primary",
                                "Configs": [
                                    {
                                        "Name": "Build1",
                                        "Skins": [],
                                        "pricol": {
                                            "t0": 0,
                                            "t1": 0,
                                            "t2": 0,
                                            "t3": 0,
                                            "m0": 0,
                                            "m1": 0,
                                            "en": 0,
                                            "e1": 0
                                        }
                                    }
                                ]
                            }
                        ],
                        "Pistols": [
                            {
                                "ItemType": "Pistol",
                                "Configs": [
                                    {
                                        "Name": "Build1",
                                        "Skins": [],
                                        "pricol": {
                                            "t0": 0,
                                            "t1": 0,
                                            "t2": 0,
                                            "t3": 0,
                                            "m0": 0,
                                            "m1": 0,
                                            "en": 0,
                                            "e1": 0
                                        }
                                    }
                                ]
                            }
                        ],
                        "Melee": [
                            {
                                "ItemType": "Melee",
                                "Configs": [
                                    {
                                        "Name": "Build1",
                                        "Skins": ["Skin1"],
                                        "pricol": {
                                            "t0": 0,
                                            "t1": 0,
                                            "t2": 0,
                                            "t3": 0,
                                            "m0": 0,
                                            "m1": 0,
                                            "en": 0,
                                            "e1": 0
                                        }
                                    }
                                ]
                            }
                        ],
                        "XPInfo": [
                            {
                                "ItemType": "Warframe",
                                "XP": 0
                            },
                            {
                                "ItemType": "Primary",
                                "XP": 0
                            },
                            {
                                "ItemType": "Secondary",
                                "XP": 0
                            },
                            {
                                "ItemType": "Melee",
                                "XP": 0
                            }
                        ]
                    },
                    "GuildId": { "$oid": "507f1f77bcf86cd799439011" },
                    "GuildName": "Guild1#456",
                    "GuildTier": 1,
                    "GuildXP": 0,
                    "GuildClass": 10,
                    "GuildEmblem": true,
                    "AllianceId": { "$oid": "507f1f77bcf86cd799439011" },
                    "PlayerSkills": {
                        "LPP_SPACE": 0,
                        "LPS_GUNNERY": 0,
                        "LPS_TACTICAL": 0,
                        "LPS_PILOTING": 0,
                        "LPS_ENGINEERING": 0,
                        "LPS_COMMAND": 0,
                        "LPP_DRIFTER": 0,
                        "LPS_DRIFT_RIDING": 0,
                        "LPS_DRIFT_COMBAT": 0,
                        "LPS_DRIFT_OPPORTUNITY": 0,
                        "LPS_DRIFT_ENDURANCE": 0
                    },
                    "ChallengeProgress": [
                        {
                            "Name": "Challenge1",
                            "Progress": 0
                        }
                    ],
                    "DeathMarks": ["DeathMark1"],
                    "Harvestable": false,
                    "DeathSquadable": false,
                    "Created": { "$date": { "$numberLong": "0" } },
                    "MigratedToConsole": false
                }
            ]
        });

        let payload: ProfilePayload = serde_json::from_value(data).expect("Deserialization failed");

        assert_eq!(payload.results.len(), 1);
        let result = &payload.results[0];

        assert_eq!(result.account_id, "507f1f77bcf86cd799439011");
        assert_eq!(result.display_name.name, "Player1");
        assert_eq!(result.display_name.platform, Platform::PC);

        assert_eq!(result.platform_names.is_some(), true);
        let platform_names = result.platform_names.as_ref().unwrap();
        assert_eq!(platform_names.len(), 5);
        assert_eq!(platform_names[0].name, "Player1");
        assert_eq!(platform_names[0].platform, Platform::PC);
        assert_eq!(platform_names[1].name, "Player1");
        assert_eq!(platform_names[1].platform, Platform::Xbox);
        assert_eq!(platform_names[2].name, "Player1");
        assert_eq!(platform_names[2].platform, Platform::PS);
        assert_eq!(platform_names[3].name, "Player1");
        assert_eq!(platform_names[3].platform, Platform::Switch);
        assert_eq!(platform_names[4].name, "Player1");
        assert_eq!(platform_names[4].platform, Platform::Ios);

        assert_eq!(result.player_level, 0);
        assert_eq!(result.guild_id.is_some(), true);

        assert_eq!(result.load_out_preset.is_some(), true);
        let load_out_preset = result.load_out_preset.as_ref().unwrap();

        assert_eq!(load_out_preset.focus_school, FocusSchool::Vazarin);
        assert_eq!(load_out_preset.preset_icon, "");
        assert_eq!(load_out_preset.favorite, false);
        assert_eq!(load_out_preset.name.as_deref(), Some("Preset1"));

        assert_eq!(load_out_preset.warframe.item_id, "507f1f77bcf86cd799439011");
        assert_eq!(load_out_preset.warframe.mod_loadout, 0);
        assert_eq!(load_out_preset.warframe.customisation_loadout, 0);

        assert_eq!(load_out_preset.primary.item_id, "507f1f77bcf86cd799439011");
        assert_eq!(load_out_preset.primary.mod_loadout, 0);
        assert_eq!(load_out_preset.primary.customisation_loadout, 0);
        assert_eq!(load_out_preset.primary.hide, false);

        assert_eq!(
            load_out_preset.secondary.item_id,
            "507f1f77bcf86cd799439011"
        );
        assert_eq!(load_out_preset.secondary.mod_loadout, 0);
        assert_eq!(load_out_preset.secondary.customisation_loadout, 0);

        assert_eq!(load_out_preset.melee.item_id, "507f1f77bcf86cd799439011");
        assert_eq!(load_out_preset.melee.mod_loadout, 0);
        assert_eq!(load_out_preset.melee.customisation_loadout, 0);

        assert_eq!(load_out_preset.h.item_id, "507f1f77bcf86cd799439011");
        assert_eq!(load_out_preset.h.mod_loadout, 0);
        assert_eq!(load_out_preset.h.customisation_loadout, 0);

        assert_eq!(result.load_out_inventory.weapon_skins.len(), 1);
        assert_eq!(result.load_out_inventory.weapon_skins[0].item_type, "Item1");

        assert_eq!(result.load_out_inventory.warframe.len(), 1);
        let warframe = &result.load_out_inventory.warframe[0];

        assert_eq!(warframe.item_type, "Warframe");

        assert_eq!(warframe.configs.len(), 1);
        let config = &warframe.configs[0];

        assert_eq!(config.base.name, "Build1");
        assert_eq!(config.base.skins.len(), 0);
        assert_eq!(config.base.primary_colors.primary, Some(0));
        assert_eq!(config.base.primary_colors.secondary, Some(0));
        assert_eq!(config.base.primary_colors.tertiary, Some(0));
        assert_eq!(config.base.primary_colors.accents, Some(0));
        assert_eq!(config.base.primary_colors.emissive_primary, Some(0));
        assert_eq!(config.base.primary_colors.emissive_secondary, None);
        assert_eq!(config.base.primary_colors.energy_primary, Some(0));
        assert_eq!(config.base.primary_colors.energy_secondary, Some(0));

        assert_eq!(config.attachment_colors.primary, Some(0));
        assert_eq!(config.attachment_colors.secondary, Some(0));
        assert_eq!(config.attachment_colors.tertiary, Some(0));
        assert_eq!(config.attachment_colors.accents, Some(0));
        assert_eq!(config.attachment_colors.emissive_primary, Some(0));
        assert_eq!(config.attachment_colors.emissive_secondary, Some(0));
        assert_eq!(config.attachment_colors.energy_primary, Some(0));
        assert_eq!(config.attachment_colors.energy_secondary, Some(0));

        assert_eq!(config.sigil_colors.front_primary, Some(0));
        assert_eq!(config.sigil_colors.front_secondary, Some(0));
        assert_eq!(config.sigil_colors.back_primary, Some(0));
        assert_eq!(config.sigil_colors.back_secondary, Some(0));
        assert_eq!(config.sigil_colors.t1, Some(0));
        assert_eq!(config.sigil_colors.t3, Some(0));
        assert_eq!(config.sigil_colors.en, Some(0));
        assert_eq!(config.sigil_colors.e1, Some(0));

        assert_eq!(result.load_out_inventory.primaries.len(), 1);
        let primary = &result.load_out_inventory.primaries[0];

        assert_eq!(primary.item_type, "Primary");

        assert_eq!(primary.configs.len(), 1);
        let config = &primary.configs[0];

        assert_eq!(config.name, "Build1");
        assert_eq!(config.skins.len(), 0);
        assert_eq!(config.primary_colors.primary, Some(0));
        assert_eq!(config.primary_colors.secondary, Some(0));
        assert_eq!(config.primary_colors.tertiary, Some(0));
        assert_eq!(config.primary_colors.accents, Some(0));
        assert_eq!(config.primary_colors.emissive_primary, Some(0));
        assert_eq!(config.primary_colors.emissive_secondary, Some(0));
        assert_eq!(config.primary_colors.energy_primary, Some(0));
        assert_eq!(config.primary_colors.energy_secondary, Some(0));

        assert_eq!(result.load_out_inventory.secondaries.len(), 1);
        let secondary = &result.load_out_inventory.secondaries[0];

        assert_eq!(secondary.item_type, "Pistol");

        assert_eq!(secondary.configs.len(), 1);
        let config = &secondary.configs[0];

        assert_eq!(config.name, "Build1");
        assert_eq!(config.skins.len(), 0);
        assert_eq!(config.primary_colors.primary, Some(0));
        assert_eq!(config.primary_colors.secondary, Some(0));
        assert_eq!(config.primary_colors.tertiary, Some(0));
        assert_eq!(config.primary_colors.accents, Some(0));
        assert_eq!(config.primary_colors.emissive_primary, Some(0));
        assert_eq!(config.primary_colors.emissive_secondary, Some(0));
        assert_eq!(config.primary_colors.energy_primary, Some(0));
        assert_eq!(config.primary_colors.energy_secondary, Some(0));

        assert_eq!(result.load_out_inventory.melee.len(), 1);
        let melee = &result.load_out_inventory.melee[0];

        assert_eq!(melee.item_type, "Melee");

        assert_eq!(melee.configs.len(), 1);
        let config = &melee.configs[0];

        assert_eq!(config.name, "Build1");
        assert_eq!(config.skins.len(), 1);
        assert_eq!(config.primary_colors.primary, Some(0));
        assert_eq!(config.primary_colors.secondary, Some(0));
        assert_eq!(config.primary_colors.tertiary, Some(0));
        assert_eq!(config.primary_colors.accents, Some(0));
        assert_eq!(config.primary_colors.emissive_primary, Some(0));
        assert_eq!(config.primary_colors.emissive_secondary, Some(0));
        assert_eq!(config.primary_colors.energy_primary, Some(0));
        assert_eq!(config.primary_colors.energy_secondary, Some(0));

        assert_eq!(result.load_out_inventory.xp_info.len(), 4);

        assert_eq!(
            result.guild_id,
            Some("507f1f77bcf86cd799439011".to_string())
        );
        assert_eq!(result.guild_name, Some("Guild1#456".to_string()));
        assert_eq!(result.guild_tier, Some(GuildTier::Ghost));
        assert_eq!(result.guild_xp, Some(0));
        assert_eq!(result.guild_class, Some(10));
        assert_eq!(result.guild_emblem, Some(true));
        assert_eq!(
            result.alliance_id,
            Some("507f1f77bcf86cd799439011".to_string())
        );

        assert_eq!(result.player_skills.len(), 11);
        assert_eq!(result.player_skills[&PlayerSkill::Railjack], 0);
        assert_eq!(result.player_skills[&PlayerSkill::RailjackGunnery], 0);
        assert_eq!(result.player_skills[&PlayerSkill::RailjackTactical], 0);
        assert_eq!(result.player_skills[&PlayerSkill::RailjackPiloting], 0);
        assert_eq!(result.player_skills[&PlayerSkill::RailjackEngineering], 0);
        assert_eq!(result.player_skills[&PlayerSkill::RailjackCommand], 0);
        assert_eq!(result.player_skills[&PlayerSkill::Drifter], 0);
        assert_eq!(result.player_skills[&PlayerSkill::DrifterRiding], 0);
        assert_eq!(result.player_skills[&PlayerSkill::DrifterCombat], 0);
        assert_eq!(result.player_skills[&PlayerSkill::DrifterOpportunity], 0);
        assert_eq!(result.player_skills[&PlayerSkill::DrifterEndurance], 0);

        assert_eq!(result.challenge_progress.len(), 1);
        assert_eq!(result.challenge_progress[0].name, "Challenge1");
        assert_eq!(result.challenge_progress[0].progress, 0);

        assert_eq!(result.death_marks.len(), 1);
        assert_eq!(result.death_marks[0], "DeathMark1");

        assert_eq!(result.harvestable, false);
        assert_eq!(result.death_squadable, false);
        assert_eq!(result.created, 0);
        assert_eq!(result.migrated_to_console, false);
    }
}
