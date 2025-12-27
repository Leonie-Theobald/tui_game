use crate::{
    item::{
        collected_item::{CollectableItem, chest_key::ChestKey},
        stationed_item::{StationaryItem, lost_chest_key::LostChestKey},
    },
    quest::IsQuest,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CollectChestKey;

impl IsQuest for CollectChestKey {
    fn get_description() -> &'static str {
        "This key on the ground looks interesting. Pick it up."
    }

    fn list_necessary_collected_items() -> Vec<CollectableItem> {
        Vec::new()
    }

    fn list_necessary_stationed_items() -> Vec<StationaryItem> {
        vec![StationaryItem::LostChestKey(LostChestKey::initiate())]
    }

    fn list_reward_collectable_items() -> Vec<CollectableItem> {
        vec![CollectableItem::ChestKey(ChestKey)]
    }

    fn list_reward_stationary_items() -> Vec<StationaryItem> {
        Vec::new()
    }
}
