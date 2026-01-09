use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TreasureMap;

impl Item for TreasureMap {
    fn get_name(&self) -> &'static str {
        "Treasure Map"
    }

    fn get_description(&self) -> &'static str {
        "An illegible map with several red X on it."
    }

    fn get_detail_view(&self) -> String {
        todo!()
    }
}

impl Collectable for TreasureMap {
    fn get_symbol(&self) -> String {
        String::from("📜")
    }
}
