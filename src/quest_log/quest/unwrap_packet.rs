use crate::{
    item::collected_item::{CollectableItem, rusty_key::RustyKey},
    quest_log::quest::{IsQuest, Quest, QuestStatus, open_rusty_lock::OpenRustyLock},
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnwrapPacket {
    pub status: QuestStatus,
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

    fn solve(&mut self, collected_items: &mut crate::item::collected_item::CollectedItems) {
        // check that all collectable items necessary to solve quest are available
        if !self
            .list_necessary_collected_items()
            .iter()
            .all(|necessary_item| collected_items.list.contains(necessary_item))
        {
            panic!("Not all necessary collected items are available")
        }

        // check that all stationary items necessary to solve quest are available
        // let necessary_stationed_items = method_on_inner_quest!(
        //     self.list[position_active_quest],
        //     list_necessary_stationed_item
        // );
        // if !necessary_stationed_items
        //     .iter()
        //     .all(|necessary_item| stationary_items.contains(&necessary_item))
        // {
        //     return Err(());
        // }

        // removed used items
        for used_item in self.list_necessary_collected_items() {
            let index = collected_items
                .list
                .iter()
                .position(|available_item| *available_item == used_item)
                .expect("Checked before that backpack contains all necessary items");
            collected_items.list.remove(index);
        }

        // quest is finished
        self.change_to_closed();

        // generate rewarded items
        collected_items.list.append(&mut self.list_reward_items());

        // generate new quest(s)
    }
}
