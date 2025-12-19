use crate::{
    item::collected_item::{
        CollectableItem, candle::Candle, rusty_key::RustyKey, treasure_map::TreasureMap,
    },
    quest_log::quest::{IsQuest, Quest, QuestStatus, light_candle::LightCandle},
};

#[derive(Clone, Debug, Default)]
pub struct OpenRustyLock {
    status: QuestStatus,
}

impl IsQuest for OpenRustyLock {
    fn get_description(&self) -> &'static str {
        "This old, wooden door is locked. Open it"
    }

    fn list_necessary_items(&self) -> &[CollectableItem] {
        &[CollectableItem::RustyKey(RustyKey)]
    }

    fn get_status(&self) -> &QuestStatus {
        &self.status
    }

    fn list_reward_items(&self) -> Vec<CollectableItem> {
        vec![
            CollectableItem::Candle(Candle),
            CollectableItem::TreasureMap(TreasureMap),
        ]
    }

    fn list_follow_up_quests(&self) -> Vec<Quest> {
        vec![Quest::LightCandle(LightCandle::default())]
    }

    fn change_to_closed(&mut self) {
        self.status = QuestStatus::Finished;
    }

    fn change_to_active(&mut self) {
        self.status = QuestStatus::Active;
    }
}
