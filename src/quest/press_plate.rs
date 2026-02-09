use serde::{Deserialize, Serialize};

use crate::{
    item::{
        collected_item::CollectedItems,
        stationed_item::{StationaryItem, StationedItems, pressure_plate::PressurePlate},
    },
    quest::{IsQuest, Quest},
};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct PressPlate {
    pub finished: bool,
}

impl IsQuest for PressPlate {
    fn get_description(&self) -> String {
        String::from("Press hidden pressure plate in wall.")
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
                StationaryItem::PressurePlate(PressurePlate { pressed: true, .. })
            )
        })
    }

    fn update_game(
        &self,
        _collected_items: &mut CollectedItems,
        _stationary_items: &mut StationedItems,
    ) -> Vec<Quest> {
        Vec::new()
    }
}
