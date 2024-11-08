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
                let v = value.as_bytes();

                if v.is_empty() {
                    return Err(E::custom("value cannot be empty"));
                }

                let platform = Platform::from_byte(
                    *v.last()
                        .ok_or_else(|| E::custom("expected a platform byte at the end"))?,
                )
                .ok_or_else(|| E::custom("Invalid platform byte"))?;

                let name = str::from_utf8(&v[..v.len() - 1])
                    .map_err(|e| E::custom(e))?
                    .to_owned();

                Ok(PlatformName { name, platform })
            }
        }

        deserializer.deserialize_str(SplitFieldVisitor)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Platform {
    PC = 0x0,
    Xbox = 0x1,
    PS = 0x2,
    Switch = 0x3,
    Ios = 0x4,
}

impl Platform {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x0 => Some(Self::PC),
            0x1 => Some(Self::Xbox),
            0x2 => Some(Self::PS),
            0x3 => Some(Self::Switch),
            0x4 => Some(Self::Ios),
            _ => None,
        }
    }
}
