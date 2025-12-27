use crate::{
    item::{
        collected_item::{CollectableItem, rusty_key::RustyKey},
        stationed_item::{
            StationaryItem, blocked_room::BlockedRoom, lost_chest_key::LostChestKey,
            treasure_chest::TreasureChest,
        },
    },
    quest::IsQuest,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpenDoor;

impl IsQuest for OpenDoor {
    fn get_description() -> &'static str {
        "Try to open with a key."
    }

    fn list_necessary_collected_items() -> Vec<CollectableItem> {
        vec![CollectableItem::RustyKey(RustyKey)]
    }

    fn list_necessary_stationed_items() -> Vec<StationaryItem> {
        vec![StationaryItem::BlockedRoom(BlockedRoom::initiate())]
    }

    fn list_reward_collectable_items() -> Vec<CollectableItem> {
        Vec::new()
    }

    fn list_reward_stationary_items() -> Vec<StationaryItem> {
        vec![
            StationaryItem::TreasureChest(TreasureChest::initiate()),
            StationaryItem::LostChestKey(LostChestKey::initiate()),
        ]
    }
}
