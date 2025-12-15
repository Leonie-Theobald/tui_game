use crate::item::{Items, RustyKey};

pub trait Quest {
    fn get_description(&self) -> &'static str;
    fn list_necessary_items(&self) -> &[Items];
    fn get_status(&self) -> &QuestStatus;
}

pub trait MutQuest {
    fn change_to_closed(&mut self);
    fn change_to_active(&mut self);
}

#[derive(Clone, Debug)]
pub enum Quests {
    OpenRustyLock(OpenRustyLock),
}

impl Quests {
    pub fn get_inner_quest(&self) -> impl Quest {
        match self {
            Self::OpenRustyLock(inner_quest) => inner_quest,
        }
    }

    pub fn get_inner_mut_quest(&mut self) -> impl MutQuest {
        match self {
            Self::OpenRustyLock(inner_quest) => inner_quest,
        }
    }
}

impl std::fmt::Display for Quests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_inner_quest().get_description())
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

impl Quest for &OpenRustyLock {
    fn get_description(&self) -> &'static str {
        "This old, wooden door is locked. Open it"
    }

    fn list_necessary_items(&self) -> &[Items] {
        &[Items::RustyKey(RustyKey)]
    }

    fn get_status(&self) -> &QuestStatus {
        &self.status
    }
}

impl MutQuest for &mut OpenRustyLock {
    fn change_to_closed(&mut self) {
        self.status = QuestStatus::Finished;
    }

    fn change_to_active(&mut self) {
        self.status = QuestStatus::Active;
    }
}
