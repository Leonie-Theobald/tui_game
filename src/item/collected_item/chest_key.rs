use crate::item::{Item, collected_item::Collectable};

#[derive(Clone, Debug, PartialEq)]
pub struct ChestKey;

impl Item for ChestKey {
    fn get_name(&self) -> &'static str {
        "Chest Key"
    }

    fn get_description(&self) -> &'static str {
        "This key belongs to a treasure chest."
    }
}

impl Collectable for ChestKey {
    fn get_symbol(&self) -> String {
        String::from("🗝️")
    }
}
