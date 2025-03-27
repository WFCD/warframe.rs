use std::{
    any::TypeId,
    sync::Arc,
};

use super::models::i18n::Language;

#[cfg(feature = "market_cache")]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CacheKey {
    type_id: TypeId,
    language: Language,

    /// Additional data to differentiate between different cache keys.
    ///
    /// This is specifically aimed at stuff like slugs.
    path_data: Arc<str>,
}

#[cfg(feature = "market_cache")]
impl CacheKey {
    pub fn new<T: 'static>(language: Language, path_data: &str) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            language,
            path_data: path_data.into(),
        }
    }

    pub fn language<T: 'static>(language: Language) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            language,
            path_data: "".into(),
        }
    }
}
