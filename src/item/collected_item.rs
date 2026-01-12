pub mod candle;
pub mod chest_key;
pub mod rusty_key;
pub mod treasure_map;

use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        collected_item::{
            candle::Candle, chest_key::ChestKey, rusty_key::RustyKey, treasure_map::TreasureMap,
        },
    },
    tui::pop_up::PopUp,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectedItems {
    pub list: Vec<CollectableItem>,
}

impl CollectedItems {
    pub fn initiate() -> Self {
        Self { list: Vec::new() }
    }

    pub fn show_all_in_popup(&self) {
        let popup_collected_item = PopUp {
            options: self.list.clone(),
        };
        if let Ok(selected_item) = popup_collected_item.get_user_selection() {
            let inspection = selected_item.get_inner().get_inspection();
            inspection.start_inspection();
            self.show_all_in_popup();
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CollectableItem {
    RustyKey(RustyKey),
    Candle(Candle),
    TreasureMap(TreasureMap),
    ChestKey(ChestKey),
}

impl CollectableItem {
    pub fn get_inner(&self) -> &dyn Collectable {
        match self {
            CollectableItem::RustyKey(inner) => inner,
            CollectableItem::Candle(inner) => inner,
            CollectableItem::TreasureMap(inner) => inner,
            CollectableItem::ChestKey(inner) => inner,
        }
    }
}

impl std::fmt::Display for CollectableItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.get_inner().get_name();
        let description = self.get_inner().get_description();

        writeln!(f, "{} - {}", name, description)
    }
}

pub trait Collectable: Item {
    fn get_symbol(&self) -> String;
}
