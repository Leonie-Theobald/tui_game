use serde::{Deserialize, Serialize};

use crate::{
    item::{
        collected_item::CollectedItems,
        stationed_item::{
            StationaryItem, StationedItems, dark_room::DarkRoom, pressure_plate::PressurePlate,
            terminal::Terminal, torch_holder::TorchHolder,
        },
    },
    quest::{IsQuest, Quest, light_room::LightRoom},
};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct UnlockTerminal {
    pub finished: bool,
}

impl IsQuest for UnlockTerminal {
    fn get_description(&self) -> String {
        String::from("Enter correct password.")
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
                StationaryItem::Terminal(Terminal {
                    correct_code_entered: true,
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
            .push(StationaryItem::DarkRoom(DarkRoom::initiate()));
        stationary_items
            .list
            .push(StationaryItem::TorchHolder(TorchHolder::initiate()));
        stationary_items
            .list
            .push(StationaryItem::PressurePlate(PressurePlate::initiate()));

        vec![Quest::LightRoom(LightRoom { finished: false })]
    }
}
