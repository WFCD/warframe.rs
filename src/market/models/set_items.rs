use serde::Deserialize;

use super::item::Item;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SetItems {
    pub id: String,

    pub items: Vec<Item>,
}
