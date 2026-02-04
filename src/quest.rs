mod find_map;
mod light_room;
mod press_plate;
mod unlock_door;

use serde::{Deserialize, Serialize};

use crate::{
    item::{collected_item::CollectedItems, stationed_item::StationedItems},
    quest::{
        find_map::FindMap, light_room::LightRoom, press_plate::PressPlate, unlock_door::UnlockDoor,
    },
    tui::pop_up::InfoPopUp,
};

#[derive(Serialize, Deserialize)]
pub enum Quest {
    UnlockDoor(UnlockDoor),
    FindMap(FindMap),
    LightRoom(LightRoom),
    PressPlate(PressPlate),
}

impl Quest {
    pub fn get_inner(&self) -> &dyn IsQuest {
        match self {
            Quest::UnlockDoor(unlock_door) => unlock_door,
            Quest::FindMap(find_map) => find_map,
            Quest::LightRoom(light_room) => light_room,
            Quest::PressPlate(press_plate) => press_plate,
        }
    }

    pub fn get_inner_mut(&mut self) -> &mut dyn IsQuest {
        match self {
            Quest::UnlockDoor(unlock_room) => unlock_room,
            Quest::FindMap(find_map) => find_map,
            Quest::LightRoom(light_room) => light_room,
            Quest::PressPlate(press_plate) => press_plate,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Quests {
    pub list: Vec<Quest>,
}

impl Quests {
    pub fn initiate() -> Self {
        Quests {
            list: vec![Quest::UnlockDoor(UnlockDoor { finished: false })],
        }
    }

    pub fn show_open_quests(&self) {
        let chained_quest_descriptions = self
            .list
            .iter()
            .filter(|quest| !quest.get_inner().is_finished())
            .fold(String::new(), |mut chained_quest_descriptions, quest| {
                chained_quest_descriptions.push_str(&quest.get_inner().get_description());
                chained_quest_descriptions.push_str("\r\n");
                chained_quest_descriptions
            });
        let info_prompt = InfoPopUp {
            message: chained_quest_descriptions,
        };
        info_prompt.show_to_user();
    }

    pub fn check_quests(
        &mut self,
        collected_items: &mut CollectedItems,
        stationed_items: &mut StationedItems,
    ) {
        let mut new_quests = Vec::new();
        for quest in &mut self.list {
            let inner_quest = quest.get_inner_mut();
            if !inner_quest.is_finished()
                && inner_quest.check_prerequisites_fulfilled(collected_items, stationed_items)
            {
                inner_quest.set_to_finished();
                new_quests.append(&mut inner_quest.update_game(collected_items, stationed_items));
            }
        }
        self.list.append(&mut new_quests);
    }
}

pub trait IsQuest {
    fn get_description(&self) -> String;
    fn is_finished(&self) -> bool;
    fn set_to_finished(&mut self);
    fn check_prerequisites_fulfilled(
        &self,
        collected_items: &CollectedItems,
        stationary_items: &StationedItems,
    ) -> bool;
    fn update_game(
        &self,
        collected_items: &mut CollectedItems,
        stationary_items: &mut StationedItems,
    ) -> Vec<Quest>;
}
