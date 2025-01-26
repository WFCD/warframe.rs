use core::str;
use std::collections::HashMap;
use serde::{
    de::self,
    Deserialize,
    Deserializer,
    Serialize,
};
use serde_json::Value;
use super::{
    guild_tier::GuildTier,
    player_skill::PlayerSkill,
    load_out_preset::LoadOutPreset,
    load_out_inventory::LoadOutInventory,
    operator_load_out::OperatorLoadOut,
    platform::PlatformName,
    affiliation::{Affiliation, AffiliationTag},
    stats::Stats,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ProfilePayload {
    pub(crate) results: Vec<Profile>,
    
    // TODO: What is this?
    //pub(crate) tech_projects: Vec<?>,
    
    // TODO: What is this?
    //pub(crate) xp_components: Vec<?>,

    #[serde(deserialize_with = "deserialize_date")]
    pub(crate) xp_cache_expiry_date: i64,
    
    pub(crate) stats: Option<Stats>
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
    pub guild_xp: Option<u32>,

    /// guild_class
    pub guild_class: Option<u8>,

    /// guild_emblem
    pub guild_emblem: Option<bool>,

    #[serde(default, deserialize_with = "deserialize_oid_or_none")]
    /// alliance_id
    pub alliance_id: Option<String>,

    #[serde(default)]
    /// player_skills
    pub player_skills: HashMap<PlayerSkill, u32>,

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
    pub daily_affiliation: u32,

    /// daily_affiliation_pvp
    pub daily_affiliation_pvp: u32,

    /// daily_affiliation_library
    pub daily_affiliation_library: u32,

    /// daily_affiliation_cetus
    pub daily_affiliation_cetus: u32,

    /// daily_affiliation_quills
    pub daily_affiliation_quills: u32,

    /// daily_affiliation_solaris
    pub daily_affiliation_solaris: u32,

    /// daily_affiliation_ventkids
    pub daily_affiliation_ventkids: u32,

    /// daily_affiliation_vox
    pub daily_affiliation_vox: u32,

    /// daily_affiliation_entrati
    pub daily_affiliation_entrati: u32,

    /// daily_affiliation_necraloid
    pub daily_affiliation_necraloid: u32,

    /// daily_affiliation_zariman
    pub daily_affiliation_zariman: u32,

    /// daily_affiliation_kahl
    pub daily_affiliation_kahl: u32,

    /// daily_affiliation_cavia
    pub daily_affiliation_cavia: u32,

    /// daily_affiliation_hex
    pub daily_affiliation_hex: u32,

    /// daily_focus
    pub daily_focus: u32,

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
pub struct ChallengeProgress {
    /// name
    pub name: String,

    /// progress
    pub progress: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mission {
    /// tag
    pub tag: String,

    /// completes
    pub completes: u32,

    /// tier - 1 is steel path completed
    pub tier: Option<u8>,
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
        
        assert_eq!(primary_colors.t0, Some(0));
        assert_eq!(primary_colors.t1, Some(0));
        assert_eq!(primary_colors.t2, Some(0));
        assert_eq!(primary_colors.t3, Some(0));
        assert_eq!(primary_colors.m0, Some(0));
        assert_eq!(primary_colors.m1, None);
        assert_eq!(primary_colors.en, Some(0));
        assert_eq!(primary_colors.e1, Some(0));

        assert!(config.attachment_colors.is_some());
        let attachment_colors = config.attachment_colors.as_ref().unwrap();
        
        assert_eq!(attachment_colors.t0, Some(0));
        assert_eq!(attachment_colors.t1, Some(0));
        assert_eq!(attachment_colors.t2, Some(0));
        assert_eq!(attachment_colors.t3, Some(0));
        assert_eq!(attachment_colors.m0, Some(0));
        assert_eq!(attachment_colors.m1, Some(0));
        assert_eq!(attachment_colors.en, Some(0));
        assert_eq!(attachment_colors.e1, Some(0));

        assert!(config.sigil_colors.is_some());
        let sigil_colors = config.sigil_colors.as_ref().unwrap();
        
        assert_eq!(sigil_colors.t0, Some(0));
        assert_eq!(sigil_colors.t1, Some(0));
        assert_eq!(sigil_colors.t2, Some(0));
        assert_eq!(sigil_colors.t3, Some(0));
        assert_eq!(sigil_colors.m0, Some(0));
        assert_eq!(sigil_colors.m1, Some(0));
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
        
        assert_eq!(primary_colors.t0, Some(0));
        assert_eq!(primary_colors.t1, Some(0));
        assert_eq!(primary_colors.t2, Some(0));
        assert_eq!(primary_colors.t3, Some(0));
        assert_eq!(primary_colors.m0, Some(0));
        assert_eq!(primary_colors.m1, Some(0));
        assert_eq!(primary_colors.en, Some(0));
        assert_eq!(primary_colors.e1, Some(0));

        assert_eq!(result.load_out_inventory.secondaries.len(), 1);
        let secondary = &result.load_out_inventory.secondaries[0];

        assert_eq!(secondary.item_type, "Pistol");

        assert_eq!(secondary.configs.len(), 1);
        let config = &secondary.configs[0];

        assert_eq!(config.name, Some("Build1".to_string()));
        assert_eq!(config.skins.len(), 0);
        
        assert!(config.primary_colors.is_some());
        let primary_colors = config.primary_colors.as_ref().unwrap();
        
        assert_eq!(primary_colors.t0, Some(0));
        assert_eq!(primary_colors.t1, Some(0));
        assert_eq!(primary_colors.t2, Some(0));
        assert_eq!(primary_colors.t3, Some(0));
        assert_eq!(primary_colors.m0, Some(0));
        assert_eq!(primary_colors.m1, Some(0));
        assert_eq!(primary_colors.en, Some(0));
        assert_eq!(primary_colors.e1, Some(0));

        assert_eq!(result.load_out_inventory.melee.len(), 1);
        let melee = &result.load_out_inventory.melee[0];

        assert_eq!(melee.item_type, "Melee");

        assert_eq!(melee.configs.len(), 1);
        let config = &melee.configs[0];

        assert_eq!(config.name, Some("Build1".to_string()));
        assert_eq!(config.skins.len(), 1);
        
        assert!(config.primary_colors.is_some());
        let primary_colors = config.primary_colors.as_ref().unwrap();
        
        assert_eq!(primary_colors.t0, Some(0));
        assert_eq!(primary_colors.t1, Some(0));
        assert_eq!(primary_colors.t2, Some(0));
        assert_eq!(primary_colors.t3, Some(0));
        assert_eq!(primary_colors.m0, Some(0));
        assert_eq!(primary_colors.m1, Some(0));
        assert_eq!(primary_colors.en, Some(0));
        assert_eq!(primary_colors.e1, Some(0));

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
        
        assert_eq!(operator_load_out_primary_colors.t0, Some(0));
        assert_eq!(operator_load_out_primary_colors.t1, None);
        assert_eq!(operator_load_out_primary_colors.t2, None);
        assert_eq!(operator_load_out_primary_colors.t3, Some(0));
        assert_eq!(operator_load_out_primary_colors.m0, None);
        assert_eq!(operator_load_out_primary_colors.m1, None);
        assert_eq!(operator_load_out_primary_colors.en, Some(0));

        assert!(operator_load_out.eye_colors.is_some());
        let operator_load_out_eye_colors = operator_load_out.eye_colors.as_ref().unwrap();
        
        assert_eq!(operator_load_out_eye_colors.t0, Some(0));
        assert_eq!(operator_load_out_eye_colors.t1, Some(0));
        assert_eq!(operator_load_out_eye_colors.t2, Some(0));
        assert_eq!(operator_load_out_eye_colors.t3, Some(0));

        assert!(operator_load_out.sigil_colors.is_some());
        let operator_load_out_sigil_colors = operator_load_out.sigil_colors.as_ref().unwrap();

        assert_eq!(operator_load_out_sigil_colors.t1, Some(0));
        assert_eq!(operator_load_out_sigil_colors.en, Some(0));

        assert!(operator_load_out.cloth_colors.is_some());
        let operator_load_out_cloth_colors = operator_load_out.cloth_colors.as_ref().unwrap();
        
        assert_eq!(operator_load_out_cloth_colors.t0, Some(0));
        assert_eq!(operator_load_out_cloth_colors.t1, Some(0));
        assert_eq!(operator_load_out_cloth_colors.t2, Some(0));
        assert_eq!(operator_load_out_cloth_colors.t3, Some(0));
        assert_eq!(operator_load_out_cloth_colors.m0, None);
        assert_eq!(operator_load_out_cloth_colors.m1, None);
        assert_eq!(operator_load_out_cloth_colors.en, Some(0));

        assert_eq!(result.unlocked_operator, true);
        assert_eq!(result.unlocked_alignment, true);

        assert!(result.alignment.is_some());
        let alignment = result.alignment.as_ref().unwrap();
        
        assert_eq!(alignment.alignment, 0);
        assert_eq!(alignment.wisdom, 0);
        
        assert_eq!(payload.xp_cache_expiry_date, 0);
        
        assert!(payload.stats.is_some());
        let stats = payload.stats.as_ref().unwrap();
        
        assert_eq!(stats.ciphers_failed, 0);
        assert_eq!(stats.ciphers_solved, 0);
        assert_eq!(stats.cipher_time, 0.0);
        assert_eq!(stats.capture_event_score, 0);
        assert_eq!(stats.deaths, 0);
        assert_eq!(stats.rating, 0);
        
        assert_eq!(stats.weapons.len(), 1);
        assert_eq!(stats.weapons[0].unique_name, "Weapon1");
        assert_eq!(stats.weapons[0].headshots, Some(0));
        assert_eq!(stats.weapons[0].hits, Some(0));
        assert_eq!(stats.weapons[0].fired, Some(0));
        assert_eq!(stats.weapons[0].kills, Some(0));
        assert_eq!(stats.weapons[0].assists, Some(0));
        assert_eq!(stats.weapons[0].xp, Some(0));
        assert_eq!(stats.weapons[0].equip_time, 0.0);
        
        assert_eq!(stats.enemies.len(), 1);
        assert_eq!(stats.enemies[0].unique_name, "Enemy1");
        assert_eq!(stats.enemies[0].executions, Some(0));
        assert_eq!(stats.enemies[0].headshots, Some(0));
        assert_eq!(stats.enemies[0].kills, Some(0));
        assert_eq!(stats.enemies[0].assists, Some(0));
        assert_eq!(stats.enemies[0].deaths, Some(0));
        
        assert_eq!(stats.heal_count, 0);
        assert_eq!(stats.income, 0);
        assert_eq!(stats.melee_kills, 0);
        assert_eq!(stats.missions_dumped, 0);
        assert_eq!(stats.missions_failed, 0);
        assert_eq!(stats.missions_interrupted, 0);
        assert_eq!(stats.missions_quit, 0);
        assert_eq!(stats.missions_completed, 0);
        
        assert_eq!(stats.missions.len(), 1);
        assert_eq!(stats.missions[0].unique_name, "Mission1");
        assert_eq!(stats.missions[0].high_score, 0);
        
        assert_eq!(stats.time_played_sec, 0.0);
        assert_eq!(stats.pickup_count, 0);
        assert_eq!(stats.player_level, 0);
        assert_eq!(stats.rank, 0);
        assert_eq!(stats.revive_count, 0);
        assert_eq!(stats.sabotage_event_score, Some(0));
        assert_eq!(stats.survival_event_score, Some(0));
        
        assert_eq!(stats.abilities.len(), 1);
        assert_eq!(stats.abilities[0].unique_name, "Ability1");
        assert_eq!(stats.abilities[0].used, 0);
        
        assert_eq!(stats.infested_event_score, Some(0));

        assert_eq!(stats.scans.len(), 1);
        assert_eq!(stats.scans[0].unique_name, "Scan1");
        assert_eq!(stats.scans[0].scans, 0);

        assert_eq!(stats.dojo_obstacle_score, Some(0));
        
        assert_eq!(stats.pvp.len(), 2);
        assert_eq!(stats.pvp[0].unique_name, "Suit1");
        assert_eq!(stats.pvp[0].suit_kills, Some(0));
        assert_eq!(stats.pvp[0].suit_deaths, Some(0));
        assert_eq!(stats.pvp[0].weapon_kills, None);
        
        assert_eq!(stats.pvp[1].unique_name, "Weapon1");
        assert_eq!(stats.pvp[1].weapon_kills, Some(0));
        assert_eq!(stats.pvp[1].suit_kills, None);
        assert_eq!(stats.pvp[1].suit_deaths, None);
        
        assert_eq!(stats.fomorian_event_score, Some(0));
        assert_eq!(stats.zephyr_score, Some(0));
        assert_eq!(stats.sentinel_game_score, Some(0));
        assert_eq!(stats.project_sinister_event_score, Some(0));
        assert_eq!(stats.pvp_games_pending_mask, Some(0));
        assert_eq!(stats.colonist_rescue_event_score_max, Some(0));
        assert_eq!(stats.ambulas_event_score_max, Some(0));
        
        assert_eq!(stats.races.len(), 1);
        assert_eq!(stats.races.get("Race1"), Some(&0));
        
        assert_eq!(stats.halloween_19_score_max, Some(0));
        assert_eq!(stats.flotilla_event_score, Some(0));
        assert_eq!(stats.flotilla_ground_badges_tier_1, Some(0));
        assert_eq!(stats.flotilla_ground_badges_tier_2, Some(0));
        assert_eq!(stats.flotilla_ground_badges_tier_3, Some(0));
        assert_eq!(stats.flotilla_space_badges_tier_1, Some(0));
        assert_eq!(stats.flotilla_space_badges_tier_2, Some(0));
        assert_eq!(stats.flotilla_space_badges_tier_3, Some(0));
        assert_eq!(stats.mech_survival_score_max, Some(0));
        assert_eq!(stats.caliber_chicks_score, Some(0));
        assert_eq!(stats.guild_name, Some("Guild1#456".to_string()));
    }
}
