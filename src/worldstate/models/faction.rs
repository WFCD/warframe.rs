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
    ManInTheWall = "Man in the Wall"
}
