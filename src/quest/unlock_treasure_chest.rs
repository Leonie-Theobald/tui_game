use crate::{
    item::{
        collected_item::{
            CollectableItem, candle::Candle, rusty_key::RustyKey, treasure_map::TreasureMap,
        },
        stationed_item::StationaryItem,
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
        vec![CollectableItem::RustyKey(RustyKey)]
    }

    fn list_necessary_stationed_items() -> Vec<StationaryItem> {
        todo!()
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
