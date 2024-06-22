use super::macros::enum_builder;
enum_builder! {
    :"A Faction in Warframe"
    Faction;
    Orokin,
    Corrupted,
    Infested,
    Infestation,
    Corpus,
    Grineer,
    Tenno,
    Narmer,
    Crossfire,
    Murmur = "The Murmur",
    :"Honestly, no idea what this is"
    ManInTheWall = "FC_MITW"
}
