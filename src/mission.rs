use crate::item::{
    collected_item::{CollectableItem, CollectedItems, treasure_map::TreasureMap},
    stationed_item::{
        StationaryItem, StationedItems, blocked_room::BlockedRoom, lost_chest_key::LostChestKey,
        treasure_chest::TreasureChest,
    },
};

pub enum Mission {
    UnlockRoom(UnlockRoom),
    FindMap(FindMap),
}

impl Mission {
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

pub struct UnlockRoom {
    pub finished: bool,
}
impl IsMission for UnlockRoom {
    fn get_description(&self) -> String {
        String::from("Remove room blockage")
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
                StationaryItem::BlockedRoom(BlockedRoom { opened: true, .. })
            )
        })
    }

    fn update_game(
        &self,
        _collected_items: &mut CollectedItems,
        stationary_items: &mut StationedItems,
    ) {
        stationary_items
            .list
            .push(StationaryItem::TreasureChest(TreasureChest::initiate()));
        stationary_items
            .list
            .push(StationaryItem::LostChestKey(LostChestKey::initiate()));
    }
}

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
