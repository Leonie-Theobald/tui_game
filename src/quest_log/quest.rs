use crate::{
    item::collected_item::CollectableItem,
    method_on_inner_quest,
    quest_log::quest::{light_candle::LightCandle, open_rusty_lock::OpenRustyLock},
};

pub mod light_candle;
pub mod open_rusty_lock;

pub trait IsQuest {
    fn get_description(&self) -> &'static str;
    fn list_necessary_items(&self) -> &[CollectableItem];
    fn get_status(&self) -> &QuestStatus;
    fn list_reward_items(&self) -> Vec<CollectableItem>;
    fn list_follow_up_quests(&self) -> Vec<Quest>;
    fn change_to_closed(&mut self);
    fn change_to_active(&mut self);
}

#[derive(Clone, Debug)]
pub enum Quest {
    OpenRustyLock(OpenRustyLock),
    LightCandle(LightCandle),
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
    Active,
    Finished,
}
