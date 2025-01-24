use core::str;
use std::fmt;

use serde::{
    de,
    de::Visitor,
    Deserialize,
    Deserializer,
    Serialize,
};

#[derive(Debug, Clone)]
pub struct PlatformName {
    /// name
    pub name: String,

    /// platform
    pub platform: Platform,
}

impl<'de> Deserialize<'de> for PlatformName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitFieldVisitor;

        impl<'de> Visitor<'de> for SplitFieldVisitor {
            type Value = PlatformName;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with a byte code for platform at the end")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut chars = value.chars();
                let platorm_char = chars.next_back().ok_or_else(|| E::custom("value cannot be empty"))?;

                let platform = Platform::from_char(platorm_char)
                    .ok_or_else(|| E::custom("Invalid platform char"))?;

                let name = chars.collect();

                Ok(PlatformName { name, platform })
            }
        }

        deserializer.deserialize_str(SplitFieldVisitor)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Platform {
    PC,
    Xbox,
    PS,
    Switch,
    Ios,
}

impl Platform {
    pub fn from_char(char: char) -> Option<Self> {
        match char {
            '\u{e000}' => Some(Self::PC),
            '\u{e001}' => Some(Self::Xbox),
            '\u{e002}' => Some(Self::PS),
            '\u{e003}' => Some(Self::Switch),
            '\u{e004}' => Some(Self::Ios),
            _ => None,
        }
    }
}
