use serde::de::DeserializeOwned;
use serde_json::error::Error;

pub trait Endpoint {
    fn get_endpoint() -> &'static str;
}

pub trait Model: DeserializeOwned {
    fn from_str(raw_json: &str) -> Result<Self, Error> {
        serde_json::from_str::<Self>(raw_json)
    }
}
