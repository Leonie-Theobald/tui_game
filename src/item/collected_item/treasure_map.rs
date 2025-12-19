use crate::item::Item;

#[derive(Clone, Debug, PartialEq)]
pub struct TreasureMap;

impl Item for TreasureMap {
    fn get_name(&self) -> &'static str {
        "Treasure Map"
    }

    fn get_description(&self) -> &'static str {
        "An illegible map with several red X on it."
    }
}
