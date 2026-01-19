use serde::{Deserialize, Serialize};

use crate::{
    item::{
        collected_item::CollectedItems,
        stationed_item::{
            StationaryItem, StationedItems, dark_room::DarkRoom,
            dropped_marshmallow::DroppedMarshmallow, extended_room::ExtendedRoom,
            fireplace::Fireplace, firewood::Firewood, locked_door::LockedDoor, oil_drum::OilDrum,
            torch_holder::TorchHolder,
        },
    },
    quest::IsQuest,
};

#[derive(Serialize, Deserialize)]
pub struct UnlockDoor {
    pub finished: bool,
}

impl IsQuest for UnlockDoor {
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
                StationaryItem::LockedDoor(LockedDoor { opened: true, .. })
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
            .push(StationaryItem::ExtendedRoom(ExtendedRoom::initiate()));
        stationary_items
            .list
            .push(StationaryItem::Fireplace(Fireplace::initiate()));
        stationary_items
            .list
            .push(StationaryItem::Firewood(Firewood::initiate()));
        stationary_items
            .list
            .push(StationaryItem::DroppedMarshmallow(
                DroppedMarshmallow::initiate(),
            ));
        stationary_items
            .list
            .push(StationaryItem::OilDrum(OilDrum::initiate()));
        stationary_items
            .list
            .push(StationaryItem::DarkRoom(DarkRoom::initiate()));
        stationary_items
            .list
            .push(StationaryItem::TorchHolder(TorchHolder::initiate()));
    }
}
