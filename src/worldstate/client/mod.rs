#[cfg(not(feature = "multilangual"))]
pub mod basic;

#[cfg(feature = "multilangual")]
pub mod multilangual;

#[derive(Default)]
pub struct Client {
    session: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Default::default()
    }
}
