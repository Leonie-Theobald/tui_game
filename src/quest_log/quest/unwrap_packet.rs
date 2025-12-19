use crate::{
    item::{
        collected_item::{CollectableItem, rusty_key::RustyKey},
        stationed_item::{StationaryItem, packet::Packet},
    },
    quest_log::quest::{IsQuest, Quest, QuestStatus, open_rusty_lock::OpenRustyLock},
};

#[derive(Clone, Debug, Default)]
pub struct UnwrapPacket {
    status: QuestStatus,
}

impl IsQuest for UnwrapPacket {
    fn get_description(&self) -> &'static str {
        "The packet is lying on the ground. Take a look"
    }

    fn list_necessary_collected_items(&self) -> Vec<CollectableItem> {
        Vec::new()
    }

    fn get_status(&self) -> &QuestStatus {
        &self.status
    }

    fn list_reward_items(&self) -> Vec<CollectableItem> {
        vec![CollectableItem::RustyKey(RustyKey)]
    }

    fn list_follow_up_quests(&self) -> Vec<Quest> {
        vec![Quest::OpenRustyLock(OpenRustyLock::default())]
    }

    fn change_to_closed(&mut self) {
        self.status = QuestStatus::Finished;
    }

    fn change_to_active(&mut self) {
        self.status = QuestStatus::Active;
    }

    fn list_necessary_stationed_item(&self) -> Vec<StationaryItem> {
        vec![StationaryItem::Packet(Packet)]
    }
}
