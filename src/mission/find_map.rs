use serde::{Deserialize, Serialize};

use crate::{
    item::{
        collected_item::{CollectableItem, CollectedItems, treasure_map::TreasureMap},
        stationed_item::StationedItems,
    },
    mission::IsMission,
};

#[derive(Serialize, Deserialize)]
pub struct FindMap {
    pub finished: bool,
}

impl IsMission for FindMap {
    fn get_description(&self) -> String {
        String::from("Find the hidden map")
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn set_to_finished(&mut self) {
        self.finished = true;
    }

    fn check_prerequisites_fulfilled(
        &self,
        collected_items: &CollectedItems,
        _stationary_items: &StationedItems,
    ) -> bool {
        collected_items
            .list
            .contains(&CollectableItem::TreasureMap(TreasureMap))
    }

    fn update_game(
        &self,
        _collected_items: &mut CollectedItems,
        _stationary_items: &mut StationedItems,
    ) {
    }
}
