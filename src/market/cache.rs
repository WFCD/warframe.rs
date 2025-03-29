use std::sync::Arc;

use super::models::i18n::Language;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheKey {
    language: Language,

    /// Additional data to differentiate between different cache keys.
    ///
    /// This is specifically aimed at stuff like slugs.
    endpoint: Arc<str>,
}

impl CacheKey {
    pub fn new(language: Language, endpoint: &str) -> Self {
        Self {
            language,
            endpoint: endpoint.into(),
        }
    }
}
