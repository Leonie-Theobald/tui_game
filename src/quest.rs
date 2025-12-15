use crate::item::{Candle, Items, RustyKey, TreasureMap};

pub trait Quest {
    fn get_description(&self) -> &'static str;
    fn list_necessary_items(&self) -> &[Items];
    fn get_status(&self) -> &QuestStatus;
    fn list_reward_items(&self) -> Vec<Items>;
    fn list_follow_up_quests(&self) -> Vec<Quests>;
}

pub trait MutQuest {
    fn change_to_closed(&mut self);
    fn change_to_active(&mut self);
}

#[derive(Clone, Debug)]
pub enum Quests {
    OpenRustyLock(OpenRustyLock),
    LightCandle(LightCandle),
}

impl Quests {
    pub fn get_inner_quest(&self) -> &dyn Quest {
        match self {
            Self::OpenRustyLock(inner_quest) => inner_quest,
            Self::LightCandle(light_candle) => light_candle,
        }
    }

    pub fn get_inner_mut_quest(&mut self) -> &mut dyn MutQuest {
        match self {
            Self::OpenRustyLock(inner_quest) => inner_quest,
            Self::LightCandle(inner_quest) => inner_quest,
        }
    }
}

impl std::fmt::Display for Quests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.get_inner_quest().get_description())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum QuestStatus {
    #[default]
    Open,
    Active,
    Finished,
}

#[derive(Clone, Debug, Default)]
pub struct OpenRustyLock {
    status: QuestStatus,
}

impl Quest for OpenRustyLock {
    fn get_description(&self) -> &'static str {
        "This old, wooden door is locked. Open it"
    }

    fn list_necessary_items(&self) -> &[Items] {
        &[Items::RustyKey(RustyKey)]
    }

    fn get_status(&self) -> &QuestStatus {
        &self.status
    }

    fn list_reward_items(&self) -> Vec<Items> {
        vec![Items::Candle(Candle), Items::TreasureMap(TreasureMap)]
    }

    fn list_follow_up_quests(&self) -> Vec<Quests> {
        vec![Quests::LightCandle(LightCandle::default())]
    }
}

impl MutQuest for OpenRustyLock {
    fn change_to_closed(&mut self) {
        self.status = QuestStatus::Finished;
    }

    fn change_to_active(&mut self) {
        self.status = QuestStatus::Active;
    }
}

#[derive(Clone, Debug, Default)]
pub struct LightCandle {
    status: QuestStatus,
}

impl Quest for LightCandle {
    fn get_description(&self) -> &'static str {
        "The candle is not burning. Try to lighten it."
    }

    fn list_necessary_items(&self) -> &[Items] {
        &[Items::Candle(Candle)]
    }

    fn get_status(&self) -> &QuestStatus {
        &self.status
    }

    fn list_reward_items(&self) -> Vec<Items> {
        Vec::new()
    }

    fn list_follow_up_quests(&self) -> Vec<Quests> {
        Vec::new()
    }
}

impl MutQuest for LightCandle {
    fn change_to_closed(&mut self) {
        self.status = QuestStatus::Finished;
    }

    fn change_to_active(&mut self) {
        self.status = QuestStatus::Active;
    }
}
