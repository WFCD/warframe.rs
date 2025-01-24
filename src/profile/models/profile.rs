use core::str;
use std::collections::HashMap;
use serde::{
    de::self,
    Deserialize,
    Deserializer,
    Serialize,
};
use serde_json::Value;
use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};
use crate::profile::models::affiliation::AffiliationTag;
use super::{
    load_out_preset::LoadOutPreset,
    platform::PlatformName,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ProfilePayload {
    pub(crate) results: Vec<Profile>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    #[serde(deserialize_with = "deserialize_oid")]
    /// account_id
    pub account_id: String,

    /// display_name
    pub display_name: PlatformName,

    #[serde(default)]
    /// platform_names
    pub platform_names: Vec<PlatformName>,

    /// player_level
    pub player_level: u8,

    /// load_out_preset
    pub load_out_preset: LoadOutPreset,

    /// load_out_inventory
    pub load_out_inventory: LoadOutInventory,

    #[serde(default, deserialize_with = "deserialize_oid_or_none")]
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

    #[serde(default, deserialize_with = "deserialize_oid_or_none")]
    /// alliance_id
    pub alliance_id: Option<String>,

    #[serde(default)]
    /// player_skills
    pub player_skills: HashMap<PlayerSkill, i32>,

    /// challenge_progress
    pub challenge_progress: Vec<ChallengeProgress>,

    #[serde(default)]
    /// death_marks
    pub death_marks: Vec<String>,

    #[serde(default)]
    /// harvestable
    pub harvestable: bool,

    #[serde(default)]
    /// death_squadable
    pub death_squadable: bool,

    #[serde(default)]
    /// accolades
    pub accolades: HashMap<String, bool>,

    #[serde(deserialize_with = "deserialize_date")]
    /// created
    pub created: i64,

    /// migrated_to_console
    pub migrated_to_console: bool,

    /// missions
    pub missions: Vec<Mission>,

    #[serde(default)]
    /// affiliations
    pub affiliations: Vec<Affiliation>,

    /// daily_affiliation
    pub daily_affiliation: i32,

    /// daily_affiliation_pvp
    pub daily_affiliation_pvp: i32,

    /// daily_affiliation_library
    pub daily_affiliation_library: i32,

    /// daily_affiliation_cetus
    pub daily_affiliation_cetus: i32,

    /// daily_affiliation_quills
    pub daily_affiliation_quills: i32,

    /// daily_affiliation_solaris
    pub daily_affiliation_solaris: i32,

    /// daily_affiliation_ventkids
    pub daily_affiliation_ventkids: i32,

    /// daily_affiliation_vox
    pub daily_affiliation_vox: i32,

    /// daily_affiliation_entrati
    pub daily_affiliation_entrati: i32,

    /// daily_affiliation_necraloid
    pub daily_affiliation_necraloid: i32,

    /// daily_affiliation_zariman
    pub daily_affiliation_zariman: i32,

    /// daily_affiliation_kahl
    pub daily_affiliation_kahl: i32,

    /// daily_affiliation_cavia
    pub daily_affiliation_cavia: i32,

    /// daily_affiliation_hex
    pub daily_affiliation_hex: i32,

    /// daily_focus
    pub daily_focus: i32,

    /// operator_load_outs
    pub operator_load_outs: Vec<OperatorLoadOut>,

    #[serde(default)]
    /// unlocked_operator
    pub unlocked_operator: bool,

    #[serde(default)]
    /// unlocked_alignment
    pub unlocked_alignment: bool,

    /// alignment
    pub alignment: Option<Alignment>
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
    pub name: Option<String>,

    /// skins
    pub skins: Vec<String>,

    #[serde(rename = "pricol")]
    /// primary_colors
    pub primary_colors: Option<ColorLoadOut>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WarframeLoadOutInventoryItemConfig {
    #[serde(flatten)]
    pub base: LoadOutInventoryItemConfig,

    #[serde(rename = "attcol")]
    /// attachment_colors
    pub attachment_colors: Option<ColorLoadOut>,

    #[serde(rename = "sigcol")]
    /// sigil_colors
    pub sigil_colors: Option<SigilColorLoadOut>,

    #[serde(rename = "syancol")]
    /// syandana_colors
    pub syandana_colors: Option<ColorLoadOut>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mission {
    /// tag
    pub tag: String,

    /// completes
    pub completes: i32,

    /// tier - 1 is steel path completed
    pub tier: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Affiliation {
    /// tag
    pub tag: AffiliationTag,

    /// standing
    pub standing: i32,

    /// title
    pub title: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OperatorLoadOut {
    /// skins
    pub skins: Vec<String>,

    /// upgrades
    pub upgrades: Option<Vec<String>>,

    /// ability_override
    pub ability_override: Option<AbilityOverride>,

    #[serde(rename = "pricol")]
    /// primary_colors
    pub primary_colors: Option<ColorLoadOut>,

    #[serde(rename = "eyecol")]
    /// eye_colors
    pub eye_colors: Option<ColorLoadOut>,

    #[serde(rename = "sigcol")]
    /// sigil_colors
    pub sigil_colors: Option<SigilColorLoadOut>,

    #[serde(rename = "cloth")]
    /// cloth_colors
    pub cloth_colors: Option<ColorLoadOut>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AbilityOverride {
    /// ability
    pub ability: String,

    /// index
    pub index: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Alignment {
    /// alignment
    pub alignment: i32,

    /// wisdom
    pub wisdom: i32,
}

pub(crate) fn deserialize_oid<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_oid_or_none(deserializer)?.ok_or_else(|| de::Error::custom("missing $oid field"))
}

pub(crate) fn deserialize_oid_or_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;

    Ok(v.get("$oid").and_then(Value::as_str).map(|s| s.to_owned()))
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;

    v.get("$date")
        .and_then(Value::as_object)
        .and_then(|date| date.get("$numberLong").and_then(Value::as_str))
        .ok_or_else(|| de::Error::custom("missing $date or $numberLong field"))?
        .parse()
        .map_err(|_| de::Error::custom("invalid $numberLong field"))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::profile::models::{
        focus_school::FocusSchool,
        platform::Platform,
    };

    #[test]
    fn test_profile_payload_deserialization() {
        let data = fs::read_to_string("test_data/profile_payload.json")
            .expect("Failed to read test data");

        let payload: ProfilePayload = serde_json::from_str(&data).expect("Deserialization failed");

        assert_eq!(payload.results.len(), 1);
        let result = &payload.results[0];

        assert_eq!(result.account_id, "507f1f77bcf86cd799439011");
        assert_eq!(result.display_name.name, "Player1");
        assert_eq!(result.display_name.platform, Platform::PC);

        assert_eq!(result.platform_names.len(), 5);
        assert_eq!(result.platform_names[0].name, "Player1");
        assert_eq!(result.platform_names[0].platform, Platform::PC);
        assert_eq!(result.platform_names[1].name, "Player1");
        assert_eq!(result.platform_names[1].platform, Platform::Xbox);
        assert_eq!(result.platform_names[2].name, "Player1");
        assert_eq!(result.platform_names[2].platform, Platform::PS);
        assert_eq!(result.platform_names[3].name, "Player1");
        assert_eq!(result.platform_names[3].platform, Platform::Switch);
        assert_eq!(result.platform_names[4].name, "Player1");
        assert_eq!(result.platform_names[4].platform, Platform::Ios);

        assert_eq!(result.player_level, 0);
        assert!(result.guild_id.is_some());

        assert_eq!(result.load_out_preset.focus_school, Some(FocusSchool::Vazarin));
        assert_eq!(result.load_out_preset.preset_icon, "");
        assert!(!result.load_out_preset.favorite);
        assert_eq!(result.load_out_preset.name.as_deref(), Some("Preset1"));

        assert_eq!(result.load_out_preset.warframe.item_id, "507f1f77bcf86cd799439011");
        assert_eq!(result.load_out_preset.warframe.mod_loadout, 0);
        assert_eq!(result.load_out_preset.warframe.customization_loadout, 0);

        assert_eq!(result.load_out_preset.primary.item_id, "507f1f77bcf86cd799439011");
        assert_eq!(result.load_out_preset.primary.mod_loadout, 0);
        assert_eq!(result.load_out_preset.primary.customization_loadout, 0);
        assert!(!result.load_out_preset.primary.hide);

        assert_eq!(
            result.load_out_preset.secondary.item_id,
            "507f1f77bcf86cd799439011"
        );
        assert_eq!(result.load_out_preset.secondary.mod_loadout, 0);
        assert_eq!(result.load_out_preset.secondary.customization_loadout, 0);

        assert_eq!(result.load_out_preset.melee.item_id, "507f1f77bcf86cd799439011");
        assert_eq!(result.load_out_preset.melee.mod_loadout, 0);
        assert_eq!(result.load_out_preset.melee.customization_loadout, 0);

        assert!(result.load_out_preset.h.is_some());
        let load_out_preset_h = result.load_out_preset.h.as_ref().unwrap();
        
        assert_eq!(load_out_preset_h.item_id, "507f1f77bcf86cd799439011");
        assert_eq!(load_out_preset_h.mod_loadout, 0);
        assert_eq!(load_out_preset_h.customization_loadout, 0);

        assert_eq!(result.load_out_inventory.weapon_skins.len(), 1);
        assert_eq!(result.load_out_inventory.weapon_skins[0].item_type, "Item1");

        assert_eq!(result.load_out_inventory.warframe.len(), 1);
        let warframe = &result.load_out_inventory.warframe[0];

        assert_eq!(warframe.item_type, "Warframe");

        assert_eq!(warframe.configs.len(), 1);
        let config = &warframe.configs[0];

        assert_eq!(config.base.name, Some("Build1".to_string()));
        assert_eq!(config.base.skins.len(), 0);
        
        assert!(config.base.primary_colors.is_some());
        let primary_colors = config.base.primary_colors.as_ref().unwrap();
        
        assert_eq!(primary_colors.primary, Some(0));
        assert_eq!(primary_colors.secondary, Some(0));
        assert_eq!(primary_colors.tertiary, Some(0));
        assert_eq!(primary_colors.accents, Some(0));
        assert_eq!(primary_colors.emissive_primary, Some(0));
        assert_eq!(primary_colors.emissive_secondary, None);
        assert_eq!(primary_colors.energy_primary, Some(0));
        assert_eq!(primary_colors.energy_secondary, Some(0));

        assert!(config.attachment_colors.is_some());
        let attachment_colors = config.attachment_colors.as_ref().unwrap();
        
        assert_eq!(attachment_colors.primary, Some(0));
        assert_eq!(attachment_colors.secondary, Some(0));
        assert_eq!(attachment_colors.tertiary, Some(0));
        assert_eq!(attachment_colors.accents, Some(0));
        assert_eq!(attachment_colors.emissive_primary, Some(0));
        assert_eq!(attachment_colors.emissive_secondary, Some(0));
        assert_eq!(attachment_colors.energy_primary, Some(0));
        assert_eq!(attachment_colors.energy_secondary, Some(0));

        assert!(config.sigil_colors.is_some());
        let sigil_colors = config.sigil_colors.as_ref().unwrap();
        
        assert_eq!(sigil_colors.front_primary, Some(0));
        assert_eq!(sigil_colors.front_secondary, Some(0));
        assert_eq!(sigil_colors.back_primary, Some(0));
        assert_eq!(sigil_colors.back_secondary, Some(0));
        assert_eq!(sigil_colors.t1, Some(0));
        assert_eq!(sigil_colors.t3, Some(0));
        assert_eq!(sigil_colors.en, Some(0));
        assert_eq!(sigil_colors.e1, Some(0));

        assert_eq!(result.load_out_inventory.primaries.len(), 1);
        let primary = &result.load_out_inventory.primaries[0];

        assert_eq!(primary.item_type, "Primary");

        assert_eq!(primary.configs.len(), 1);
        let config = &primary.configs[0];

        assert_eq!(config.name, Some("Build1".to_string()));
        assert_eq!(config.skins.len(), 0);
        
        assert!(config.primary_colors.is_some());
        let primary_colors = config.primary_colors.as_ref().unwrap();
        
        assert_eq!(primary_colors.primary, Some(0));
        assert_eq!(primary_colors.secondary, Some(0));
        assert_eq!(primary_colors.tertiary, Some(0));
        assert_eq!(primary_colors.accents, Some(0));
        assert_eq!(primary_colors.emissive_primary, Some(0));
        assert_eq!(primary_colors.emissive_secondary, Some(0));
        assert_eq!(primary_colors.energy_primary, Some(0));
        assert_eq!(primary_colors.energy_secondary, Some(0));

        assert_eq!(result.load_out_inventory.secondaries.len(), 1);
        let secondary = &result.load_out_inventory.secondaries[0];

        assert_eq!(secondary.item_type, "Pistol");

        assert_eq!(secondary.configs.len(), 1);
        let config = &secondary.configs[0];

        assert_eq!(config.name, Some("Build1".to_string()));
        assert_eq!(config.skins.len(), 0);
        
        assert!(config.primary_colors.is_some());
        let primary_colors = config.primary_colors.as_ref().unwrap();
        
        assert_eq!(primary_colors.primary, Some(0));
        assert_eq!(primary_colors.secondary, Some(0));
        assert_eq!(primary_colors.tertiary, Some(0));
        assert_eq!(primary_colors.accents, Some(0));
        assert_eq!(primary_colors.emissive_primary, Some(0));
        assert_eq!(primary_colors.emissive_secondary, Some(0));
        assert_eq!(primary_colors.energy_primary, Some(0));
        assert_eq!(primary_colors.energy_secondary, Some(0));

        assert_eq!(result.load_out_inventory.melee.len(), 1);
        let melee = &result.load_out_inventory.melee[0];

        assert_eq!(melee.item_type, "Melee");

        assert_eq!(melee.configs.len(), 1);
        let config = &melee.configs[0];

        assert_eq!(config.name, Some("Build1".to_string()));
        assert_eq!(config.skins.len(), 1);
        
        assert!(config.primary_colors.is_some());
        let primary_colors = config.primary_colors.as_ref().unwrap();
        
        assert_eq!(primary_colors.primary, Some(0));
        assert_eq!(primary_colors.secondary, Some(0));
        assert_eq!(primary_colors.tertiary, Some(0));
        assert_eq!(primary_colors.accents, Some(0));
        assert_eq!(primary_colors.emissive_primary, Some(0));
        assert_eq!(primary_colors.emissive_secondary, Some(0));
        assert_eq!(primary_colors.energy_primary, Some(0));
        assert_eq!(primary_colors.energy_secondary, Some(0));

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

        assert_eq!(result.accolades.len(), 1);
        assert_eq!(result.accolades.get("Accolade1"), Some(&true));

        assert_eq!(result.created, 0);
        assert!(!result.migrated_to_console);

        assert_eq!(result.missions.len(), 2);
        assert_eq!(result.missions[0].tag, "Mission1");
        assert_eq!(result.missions[0].completes, 3);
        assert_eq!(result.missions[0].tier, Some(1));

        assert_eq!(result.missions[1].tag, "Mission2");
        assert_eq!(result.missions[1].completes, 4);
        assert_eq!(result.missions[1].tier, None);

        assert_eq!(result.affiliations.len(), 1);
        assert_eq!(result.affiliations[0].tag, AffiliationTag::ArbitersOfHexis);
        assert_eq!(result.affiliations[0].standing, 200000);
        assert_eq!(result.affiliations[0].title, 4);

        assert_eq!(result.daily_affiliation, 0);
        assert_eq!(result.daily_affiliation_pvp, 0);
        assert_eq!(result.daily_affiliation_library, 0);
        assert_eq!(result.daily_affiliation_cetus, 0);
        assert_eq!(result.daily_affiliation_quills, 0);
        assert_eq!(result.daily_affiliation_solaris, 0);
        assert_eq!(result.daily_affiliation_ventkids, 0);
        assert_eq!(result.daily_affiliation_vox, 0);
        assert_eq!(result.daily_affiliation_entrati, 0);
        assert_eq!(result.daily_affiliation_necraloid, 0);
        assert_eq!(result.daily_affiliation_zariman, 0);
        assert_eq!(result.daily_affiliation_kahl, 0);
        assert_eq!(result.daily_affiliation_cavia, 0);
        assert_eq!(result.daily_affiliation_hex, 0);
        assert_eq!(result.daily_focus, 0);

        assert_eq!(result.operator_load_outs.len(), 1);
        let operator_load_out = &result.operator_load_outs[0];

        assert_eq!(operator_load_out.skins.len(), 1);
        assert_eq!(operator_load_out.skins[0], "Skin1");

        assert!(operator_load_out.upgrades.is_some());
        let operator_load_out_upgrades = operator_load_out.upgrades.as_ref().unwrap();
        
        assert_eq!(operator_load_out_upgrades.len(), 1);
        assert_eq!(operator_load_out_upgrades[0], "5f24b0df01109467953ec82b");

        assert!(operator_load_out.ability_override.is_some());
        let operator_load_out_ability_override = operator_load_out.ability_override.as_ref().unwrap();
        
        assert_eq!(operator_load_out_ability_override.ability, "Ability1");
        assert_eq!(operator_load_out_ability_override.index, 0);

        assert!(operator_load_out.primary_colors.is_some());
        let operator_load_out_primary_colors = operator_load_out.primary_colors.as_ref().unwrap();
        
        assert_eq!(operator_load_out_primary_colors.primary, Some(0));
        assert_eq!(operator_load_out_primary_colors.secondary, None);
        assert_eq!(operator_load_out_primary_colors.tertiary, None);
        assert_eq!(operator_load_out_primary_colors.accents, Some(0));
        assert_eq!(operator_load_out_primary_colors.emissive_primary, None);
        assert_eq!(operator_load_out_primary_colors.emissive_secondary, None);
        assert_eq!(operator_load_out_primary_colors.energy_primary, Some(0));

        assert!(operator_load_out.eye_colors.is_some());
        let operator_load_out_eye_colors = operator_load_out.eye_colors.as_ref().unwrap();
        
        assert_eq!(operator_load_out_eye_colors.primary, Some(0));
        assert_eq!(operator_load_out_eye_colors.secondary, Some(0));
        assert_eq!(operator_load_out_eye_colors.tertiary, Some(0));
        assert_eq!(operator_load_out_eye_colors.accents, Some(0));

        assert!(operator_load_out.sigil_colors.is_some());
        let operator_load_out_sigil_colors = operator_load_out.sigil_colors.as_ref().unwrap();
        
        assert_eq!(operator_load_out_sigil_colors.front_primary, None);
        assert_eq!(operator_load_out_sigil_colors.front_secondary, None);
        assert_eq!(operator_load_out_sigil_colors.back_primary, None);
        assert_eq!(operator_load_out_sigil_colors.back_secondary, None);
        assert_eq!(operator_load_out_sigil_colors.t1, Some(0));
        assert_eq!(operator_load_out_sigil_colors.t3, None);
        assert_eq!(operator_load_out_sigil_colors.en, Some(0));

        assert!(operator_load_out.cloth_colors.is_some());
        let operator_load_out_cloth_colors = operator_load_out.cloth_colors.as_ref().unwrap();
        
        assert_eq!(operator_load_out_cloth_colors.primary, Some(0));
        assert_eq!(operator_load_out_cloth_colors.secondary, Some(0));
        assert_eq!(operator_load_out_cloth_colors.tertiary, Some(0));
        assert_eq!(operator_load_out_cloth_colors.accents, Some(0));
        assert_eq!(operator_load_out_cloth_colors.emissive_primary, None);
        assert_eq!(operator_load_out_cloth_colors.emissive_secondary, None);
        assert_eq!(operator_load_out_cloth_colors.energy_primary, Some(0));

        assert_eq!(result.unlocked_operator, true);
        assert_eq!(result.unlocked_alignment, true);

        assert!(result.alignment.is_some());
        let alignment = result.alignment.as_ref().unwrap();
        
        assert_eq!(alignment.alignment, 0);
        assert_eq!(alignment.wisdom, 0);
    }
}
