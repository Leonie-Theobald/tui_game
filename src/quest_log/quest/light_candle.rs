use crate::{
    item::collected_item::{CollectableItem, candle::Candle},
    quest_log::quest::{IsQuest, Quest, QuestStatus},
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LightCandle {
    status: QuestStatus,
}

impl IsQuest for LightCandle {
    fn get_description(&self) -> &'static str {
        "The candle is not burning. Try to lighten it."
    }

    fn list_necessary_collected_items(&self) -> Vec<CollectableItem> {
        vec![CollectableItem::Candle(Candle)]
    }

    fn get_status(&self) -> &QuestStatus {
        &self.status
    }

    fn list_reward_items(&self) -> Vec<CollectableItem> {
        Vec::new()
    }

    fn list_follow_up_quests(&self) -> Vec<Quest> {
        Vec::new()
    }

    fn change_to_closed(&mut self) {
        self.status = QuestStatus::Finished;
    }

    fn change_to_active(&mut self) {
        self.status = QuestStatus::Active;
    }
}
