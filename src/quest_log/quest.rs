use crate::{
    item::{
        collected_item::{CollectableItem, CollectedItems},
        stationed_item::StationaryItem,
    },
    method_on_inner_quest,
    quest_log::quest::{unlock_treasure_chest::UnlockTreasureChest, unwrap_packet::UnwrapPacket},
};

pub mod unlock_treasure_chest;
pub mod unwrap_packet;

pub trait IsQuest {
    fn get_description(&self) -> &'static str;
    fn list_necessary_collected_items(&self) -> Vec<CollectableItem>;
    fn list_reward_items(&self) -> Vec<CollectableItem>;
    fn solve(&mut self, collected_items: &mut CollectedItems) -> Vec<StationaryItem>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Quest {
    UnwrapPacket(UnwrapPacket),
    UnlockTreasureChest(UnlockTreasureChest),
}

impl std::fmt::Display for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", method_on_inner_quest!(self, get_description))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum QuestStatus {
    #[default]
    Open,
    Finished,
}

#[macro_export]
macro_rules! method_on_inner_quest {
    ($value:expr, $method:ident) => {
        match &$value {
            Quest::UnwrapPacket(inner) => inner.$method(),
            Quest::UnlockTreasureChest(inner) => inner.$method(),
        }
    };
}

#[macro_export]
macro_rules! mut_method_on_inner_quest {
    ($value:expr, $method:ident) => {
        match $value {
            Quest::UnwrapPacket(inner) => inner.$method(),
            Quest::UnlockTreasureChest(inner) => inner.$method(),
        }
    };
}
