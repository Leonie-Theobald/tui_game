use serde::{Deserialize, Serialize};

use crate::{
    item::{
        collected_item::{CollectableItem, CollectedItems, treasure_map::TreasureMap},
        stationed_item::{StationaryItem, StationedItems},
    },
    quest::{IsQuest, Quest, press_plate::PressPlate},
};

#[derive(Serialize, Deserialize)]
pub struct FindMap {
    pub finished: bool,
}

impl IsQuest for FindMap {
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
        stationary_items: &mut StationedItems,
    ) -> Vec<Quest> {
        for item in stationary_items.list.iter_mut() {
            match item {
                StationaryItem::PressurePlate(pressure_plate) => pressure_plate.activated = true,
                _ => continue,
            }
        }

        vec![Quest::PressPlate(PressPlate { finished: false })]
    }
}
