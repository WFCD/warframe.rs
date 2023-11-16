pub mod client;
pub mod error;
pub mod models;

pub mod prelude {
    pub use crate::worldstate::client::Client;
    pub use crate::worldstate::error::ApiError;
    pub use crate::worldstate::models::*;
}
