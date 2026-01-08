mod find_map;
mod unlock_room;

use serde::{Deserialize, Serialize};

use crate::{
    item::{collected_item::CollectedItems, stationed_item::StationedItems},
    mission::{find_map::FindMap, unlock_room::UnlockRoom},
};

#[derive(Serialize, Deserialize)]
pub enum Mission {
    UnlockRoom(UnlockRoom),
    FindMap(FindMap),
}

impl Mission {
    pub fn initiate() -> Vec<Self> {
        vec![
            Mission::UnlockRoom(UnlockRoom { finished: false }),
            Mission::FindMap(FindMap { finished: false }),
        ]
    }
    pub fn get_inner(&self) -> &dyn IsMission {
        match self {
            Mission::UnlockRoom(unlock_room) => unlock_room,
            Mission::FindMap(find_map) => find_map,
        }
    }

    pub fn get_inner_mut(&mut self) -> &mut dyn IsMission {
        match self {
            Mission::UnlockRoom(unlock_room) => unlock_room,
            Mission::FindMap(find_map) => find_map,
        }
    }

    pub fn check_missions(
        missions: &mut Vec<Mission>,
        collected_items: &mut CollectedItems,
        stationed_items: &mut StationedItems,
    ) {
        for mission in missions {
            let inner_mission = mission.get_inner_mut();
            if !inner_mission.is_finished()
                && inner_mission.check_prerequisites_fulfilled(collected_items, stationed_items)
            {
                inner_mission.set_to_finished();
                inner_mission.update_game(collected_items, stationed_items);
            }
        }
    }
}

pub trait IsMission {
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
    );
}
