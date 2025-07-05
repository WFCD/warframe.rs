use serde::Deserialize;

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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum AffiliationTag {
    #[serde(rename = "ArbitersSyndicate")]
    ArbitersOfHexis,
    #[serde(rename = "CephalonSudaSyndicate")]
    CephalonSuda,
    #[serde(rename = "NewLokaSyndicate")]
    NewLoka,
    #[serde(rename = "PerrinSyndicate")]
    PerrinSequence,
    #[serde(rename = "RedVeilSyndicate")]
    RedVeil,
    #[serde(rename = "SteelMeridianSyndicate")]
    SteelMeridian,
    #[serde(rename = "LibrarySyndicate")]
    CephalonSimaris,
    #[serde(rename = "ConclaveSyndicate")]
    Conclave,
    #[serde(rename = "CetusSyndicate")]
    Ostrons,
    #[serde(rename = "EventSyndicate")]
    PlagueStar,
    #[serde(rename = "QuillsSyndicate")]
    Quills,
    #[serde(rename = "VentKidsSyndicate")]
    VentKids,
    #[serde(rename = "SolarisSyndicate")]
    SolarisUnited,
    #[serde(rename = "VoxSyndicate")]
    VoxSolaris,
    #[serde(rename = "RadioLegionSyndicate")]
    NightwaveSaturnSix,
    #[serde(rename = "RadioLegionIntermissionSyndicate")]
    NightwaveIntermission1,
    #[serde(rename = "RadioLegion2Syndicate")]
    NightwaveEmissary,
    #[serde(rename = "RadioLegionIntermission2Syndicate")]
    NightwaveIntermission2,
    #[serde(rename = "RadioLegion3Syndicate")]
    NightwaveGlassmaker,
    #[serde(rename = "EntratiSyndicate")]
    Entrati,
    #[serde(rename = "NecraloidSyndicate")]
    Necarloid,
    #[serde(rename = "RadioLegionIntermission3Syndicate")]
    NightwaveIntermission3,
    #[serde(rename = "RadioLegionIntermission4Syndicate")]
    NightwaveNorasChoice,
    #[serde(rename = "RadioLegionIntermission5Syndicate")]
    NightwaveNorasMix1,
    #[serde(rename = "ZarimanSyndicate")]
    Zariman,
    #[serde(rename = "RadioLegionIntermission6Syndicate")]
    NightwaveNorasMix2,
    #[serde(rename = "KahlSyndicate")]
    KahlsGarrison,
    #[serde(rename = "RadioLegionIntermission7Syndicate")]
    NightwaveNorasMix3,
    #[serde(rename = "RadioLegionIntermission8Syndicate")]
    NightwaveNorasMix4,
    #[serde(rename = "RadioLegionIntermission9Syndicate")]
    NightwaveNorasMix5,
    #[serde(rename = "EntratiLabSyndicate")]
    Cavia,
    #[serde(rename = "RadioLegionIntermission10Syndicate")]
    NightwaveNorasMix6,
    #[serde(rename = "RadioLegionIntermission11Syndicate")]
    NightwaveNorasMix7,
    #[serde(rename = "HexSyndicate")]
    Hex,
}
