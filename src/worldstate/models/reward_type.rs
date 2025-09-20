#![allow(clippy::doc_markdown)]
use warframe_macros::model;

/// Represents different reward types
#[model]
#[serde(rename_all = "camelCase")]
pub enum RewardType {
    /// Vauban
    Vauban,

    /// Vandal
    Vandal,

    /// Wraith
    Wraith,

    /// Skin
    Skin,

    /// Helmet
    Helmet,

    /// Nitain
    Nitain,

    /// Mutalist
    Mutalist,

    /// Weapon
    Weapon,

    /// Fieldron
    Fieldron,

    /// Detonite
    Detonite,

    /// Mutagen
    Mutagen,

    /// Aura
    Aura,

    /// NeuralSensors
    NeuralSensors,

    /// OrokinCell
    OrokinCell,

    /// AlloyPlate
    AlloyPlate,

    /// Circuits
    Circuits,

    /// ControlModule
    ControlModule,

    /// Ferrite
    Ferrite,

    /// Gallium
    Gallium,

    /// Morphics
    Morphics,

    /// NanoSpores
    NanoSpores,

    /// Oxium
    Oxium,

    /// Rubedo
    Rubedo,

    /// Salvage
    Salvage,

    /// Plastids
    Plastids,

    /// PolymerBundle
    PolymerBundle,

    /// ArgonCrystal
    ArgonCrystal,

    /// Cryotic
    Cryotic,

    /// Tellurium
    Tellurium,

    /// Neurodes
    Neurodes,

    /// Nightmare
    Nightmare,

    /// Endo
    Endo,

    /// Reactor
    Reactor,

    /// Catalyst
    Catalyst,

    /// Forma
    Forma,

    /// Synthula
    Synthula,

    /// Exilus
    Exilus,

    /// Riven
    Riven,

    /// KavatGene
    KavatGene,

    /// KubrowEgg
    KubrowEgg,

    /// Traces
    Traces,

    /// Other
    Other,

    /// Credits
    Credits,
}
