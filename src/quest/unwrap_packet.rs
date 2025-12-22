use crate::{
    item::{
        collected_item::{CollectableItem, CollectedItems, rusty_key::RustyKey},
        stationed_item::{StationaryItem, treasure_chest::TreasureChest},
    },
    quest::{IsQuest, QuestError, QuestStatus},
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

    fn list_reward_collectable_items(&self) -> Vec<CollectableItem> {
        vec![CollectableItem::RustyKey(RustyKey)]
    }

    fn list_reward_stationary_items(&self) -> Vec<StationaryItem> {
        vec![StationaryItem::TreasureChest(TreasureChest::initiate())]
    }

    fn try_solve(&mut self, collected_items: &mut CollectedItems) -> Result<(), QuestError> {
        // check that all collectable items necessary to solve quest are available
        if !self
            .list_necessary_collected_items()
            .iter()
            .all(|necessary_item| collected_items.list.contains(necessary_item))
        {
            return Err(QuestError::CollectableItemMissing);
        }

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
        self.status = QuestStatus::Finished;

        // generate rewarded items
        collected_items
            .list
            .append(&mut self.list_reward_collectable_items());

        // return new stationed items
        // vec![StationaryItem::TreasureChest(TreasureChest::initiate())]
        Ok(())
    }
}
