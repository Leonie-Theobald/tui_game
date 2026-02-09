use serde::{Deserialize, Serialize};

use crate::{
    item::{
        collected_item::CollectedItems,
        stationed_item::{
            StationaryItem, StationedItems, torch_holder::TorchHolder,
            treasure_chest::TreasureChest,
        },
    },
    quest::{IsQuest, Quest, find_map::FindMap},
};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct LightRoom {
    pub finished: bool,
}

impl IsQuest for LightRoom {
    fn get_description(&self) -> String {
        String::from("Light the dark room.")
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn set_to_finished(&mut self) {
        self.finished = true;
    }

    fn check_prerequisites_fulfilled(
        &self,
        _collected_items: &CollectedItems,
        stationary_items: &StationedItems,
    ) -> bool {
        stationary_items.list.iter().any(|item| {
            matches!(
                item,
                StationaryItem::TorchHolder(TorchHolder {
                    holds_torch: true,
                    ..
                })
            )
        })
    }

    fn update_game(
        &self,
        _collected_items: &mut CollectedItems,
        stationary_items: &mut StationedItems,
    ) -> Vec<Quest> {
        stationary_items
            .list
            .push(StationaryItem::TreasureChest(TreasureChest::initiate()));

        vec![Quest::FindMap(FindMap { finished: false })]
    }
}
