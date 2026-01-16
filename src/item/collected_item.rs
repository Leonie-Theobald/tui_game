pub mod branch;
pub mod candle;
pub mod chest_key;
pub mod marshmallow;
pub mod oiled_branch;
pub mod roasted_marshmallow;
pub mod rusty_key;
pub mod skewer;
pub mod torch;
pub mod treasure_map;

use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        collected_item::{
            branch::Branch, candle::Candle, chest_key::ChestKey, marshmallow::Marshmallow,
            oiled_branch::OiledBranch, roasted_marshmallow::RoastedMarshmallow,
            rusty_key::RustyKey, skewer::Skewer, torch::Torch, treasure_map::TreasureMap,
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
    Branch(Branch),
    Skewer(Skewer),
    OiledBranch(OiledBranch),
    Torch(Torch),
    Marshmallow(Marshmallow),
    RoastedMarshmallow(RoastedMarshmallow),
}

impl CollectableItem {
    pub fn get_inner(&self) -> &dyn Collectable {
        match self {
            CollectableItem::RustyKey(inner) => inner,
            CollectableItem::Candle(inner) => inner,
            CollectableItem::TreasureMap(inner) => inner,
            CollectableItem::ChestKey(inner) => inner,
            CollectableItem::Branch(inner) => inner,
            CollectableItem::Skewer(inner) => inner,
            CollectableItem::OiledBranch(inner) => inner,
            CollectableItem::Torch(inner) => inner,
            CollectableItem::Marshmallow(inner) => inner,
            CollectableItem::RoastedMarshmallow(inner) => inner,
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
