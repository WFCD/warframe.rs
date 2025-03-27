use serde::Deserialize;

use super::item::Item;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SetItems {
    id: String,

    items: Vec<Item>,
}
