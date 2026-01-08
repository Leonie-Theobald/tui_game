use serde::{Deserialize, Serialize};

use crate::{
    item::{
        collected_item::CollectedItems,
        stationed_item::{
            StationaryItem, StationedItems, blocked_room::BlockedRoom,
            lost_chest_key::LostChestKey, treasure_chest::TreasureChest,
        },
    },
    mission::IsMission,
};

#[derive(Serialize, Deserialize)]
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
