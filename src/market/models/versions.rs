use serde::Deserialize;

use super::impl_queryable;

impl_queryable!(Versions, Object, "/versions");

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    pub id: String,
    pub apps: Apps,
    pub collections: Collections,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apps {
    pub ios: String,
    pub android: String,
    pub min_ios: String,
    pub min_android: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Collections {
    pub items: String,

    pub rivens: String,

    pub liches: String,

    pub sisters: String,

    pub missions: String,

    pub npcs: String,

    pub locations: String,
}

#[cfg(test)]
mod test {
    use super::Versions;
    use crate::market::models::ResponseBase;

    #[rstest::rstest]
    fn test_versions(
        #[files("src/market/models/fixtures/versions.json")]
        #[mode = str]
        json: &str,
    ) -> Result<(), serde_json::Error> {
        serde_json::from_str::<ResponseBase<Versions>>(json)?;

        Ok(())
    }
}
