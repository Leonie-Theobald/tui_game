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
        #[expect(
            unused_must_use,
            reason = "We don't need user's selection here. 
                    It is just the API to present the items to the user."
        )]
        popup_collected_item.get_user_selection();
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CollectableItem {
    RustyKey(RustyKey),
    Candle(Candle),
    TreasureMap(TreasureMap),
    ChestKey(ChestKey),
}

#[macro_export]
macro_rules! method_on_inner_collected_item {
    ($value:expr, $method:ident) => {
        match $value {
            CollectableItem::RustyKey(inner) => inner.$method(),
            CollectableItem::Candle(inner) => inner.$method(),
            CollectableItem::TreasureMap(inner) => inner.$method(),
            CollectableItem::ChestKey(inner) => inner.$method(),
        }
    };
}

impl std::fmt::Display for CollectableItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = method_on_inner_collected_item!(self, get_name);
        let description = method_on_inner_collected_item!(self, get_description);

        writeln!(f, "{} - {}", name, description)
    }
}

pub trait Collectable: Item {
    fn get_symbol(&self) -> String;
}
