use crate::{
    item::{
        collected_item::{
            CollectableItem, candle::Candle, chest_key::ChestKey, treasure_map::TreasureMap,
        },
        stationed_item::{StationaryItem, treasure_chest::TreasureChest},
    },
    quest::IsQuest,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnlockTreasureChest;

impl IsQuest for UnlockTreasureChest {
    fn get_description() -> &'static str {
        "The chest is locked. Try to open it."
    }

    fn list_necessary_collected_items() -> Vec<CollectableItem> {
        vec![CollectableItem::ChestKey(ChestKey)]
    }

    fn list_necessary_stationed_items() -> Vec<StationaryItem> {
        vec![StationaryItem::TreasureChest(TreasureChest::initiate())]
    }

    fn list_reward_collectable_items() -> Vec<CollectableItem> {
        vec![
            CollectableItem::Candle(Candle),
            CollectableItem::TreasureMap(TreasureMap),
        ]
    }

    fn list_reward_stationary_items() -> Vec<StationaryItem> {
        Vec::new()
    }
}
