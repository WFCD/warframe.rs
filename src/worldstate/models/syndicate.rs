use warframe_macros::model;

/// A Syndicate in Warframe
#[model]
pub enum Syndicate {
    /// ArbitersOfHexis
    #[serde(rename = "Arbiters of Hexis")]
    ArbitersOfHexis,
    /// CephalonSuda
    #[serde(rename = "Cephalon Suda")]
    CephalonSuda,
    /// Assassins
    #[serde(rename = "Assassins")]
    Assassins,
    /// Nightwave
    #[serde(rename = "Nightwave")]
    Nightwave,
    /// Ostrons
    #[serde(rename = "Ostrons")]
    Ostrons,
    /// VoxSolaris
    #[serde(rename = "Vox Solaris")]
    VoxSolaris,
    /// SolarisUnited
    #[serde(rename = "Solaris United")]
    SolarisUnited,
    /// PerrinSequence
    #[serde(rename = "Perrin Sequence")]
    PerrinSequence,
    /// SteelMeridian
    #[serde(rename = "Steel Meridian")]
    SteelMeridian,
    /// RedVeil
    #[serde(rename = "Red Veil")]
    RedVeil,
    /// NewLoka
    #[serde(rename = "New Loka")]
    NewLoka,
    /// Holdfasts
    #[serde(rename = "The Holdfasts")]
    Holdfasts,
    /// Entrati
    Entrati,
    /// Cavia
    #[serde(rename = "EntratiLabSyndicate")]
    Cavia,
    /// VentKids
    #[serde(rename = "Operations Syndicate")]
    VentKids,
    /// KahlsGarrison
    #[serde(rename = "Kahl's Garrison")]
    KahlsGarrison,
    /// Necraloid
    Necraloid,
}
