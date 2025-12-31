pub mod collect_chest_key;
pub mod open_door;
#[cfg(test)]
mod tests;
pub mod unlock_treasure_chest;
pub mod unwrap_packet;

use crate::{
    item::{
        collected_item::{CollectableItem, CollectedItems},
        reward_items::RewardItems,
        stationed_item::{StationaryItem, StationedItems},
    },
    method_on_inner_quest,
    quest::{
        collect_chest_key::CollectChestKey, open_door::OpenDoor,
        unlock_treasure_chest::UnlockTreasureChest, unwrap_packet::UnwrapPacket,
    },
};

pub trait IsQuest {
    fn get_description() -> &'static str;
    fn list_necessary_collected_items() -> Vec<CollectableItem>;
    fn list_necessary_stationed_items() -> Vec<StationaryItem>;
    fn list_reward_collectable_items() -> Vec<CollectableItem>;
    fn list_reward_stationary_items() -> Vec<StationaryItem>;
    fn necessary_items_available<T: std::cmp::PartialEq>(
        available_items: &[T],
        necessary_items: &[T],
    ) -> bool {
        necessary_items
            .iter()
            .all(|necessary_item| available_items.contains(necessary_item))
    }
    fn check_if_solvable(
        collected_items: &CollectedItems,
        stationed_items: &StationedItems,
    ) -> Result<RewardItems, QuestError> {
        let mut reward_items = RewardItems::default();

        // check that all collectable items necessary to solve quest are available
        if !Self::necessary_items_available(
            &collected_items.list,
            &Self::list_necessary_collected_items(),
        ) {
            return Err(QuestError::CollectableItemMissing);
        }

        // check that all collectable items necessary to solve quest are available
        if !Self::necessary_items_available(
            &stationed_items.list,
            &Self::list_necessary_stationed_items(),
        ) {
            return Err(QuestError::StationaryItemMissing);
        }

        // removed used items
        reward_items.removed_collectable = Self::list_necessary_collected_items();
        reward_items.removed_stationed = Self::list_necessary_stationed_items();

        // generate rewarded items
        reward_items.added_collectable = Self::list_reward_collectable_items();
        reward_items.added_stationed = Self::list_reward_stationary_items();

        Ok(reward_items)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Quest {
    UnwrapPacket(UnwrapPacket),
    UnlockTreasureChest(UnlockTreasureChest),
    OpenDoor(OpenDoor),
    CollectChestKey(CollectChestKey),
}

impl std::fmt::Display for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", method_on_inner_quest!(self, get_description))
    }
}

#[derive(Debug, PartialEq)]
pub enum QuestError {
    CollectableItemMissing,
    StationaryItemMissing,
}

#[macro_export]
macro_rules! method_on_inner_quest {
    // call without parameters
    ($value:expr, $method:ident) => {
        match &$value {
            Quest::UnwrapPacket(_inner) => UnwrapPacket::$method(),
            Quest::UnlockTreasureChest(_inner) => UnlockTreasureChest::$method(),
            Quest::OpenDoor(_inner) => OpenDoor::$method(),
            Quest::CollectChestKey(_inner) => CollectChestKey::$method(),
        }
    };
    // call with parameters
    ($value:expr, $method:ident ($ ( $parameter:expr ),* $(,)? )) => {
        match &$value {
            Quest::UnwrapPacket(_inner) => UnwrapPacket::$method( $( $parameter ),* ),
            Quest::UnlockTreasureChest(_inner) => UnlockTreasureChest::$method( $( $parameter ),*),
            Quest::OpenDoor(_inner) => OpenDoor::$method( $( $parameter ),* ),
            Quest::CollectChestKey(_inner) => CollectChestKey::$method( $( $parameter ),* ),
        }
    };
}
