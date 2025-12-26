use crate::{
    item::{
        collected_item::{CollectableItem, CollectedItems, rusty_key::RustyKey},
        reward_items::RewardItems,
        stationed_item::{
            StationaryItem, StationedItems, packet::Packet, treasure_chest::TreasureChest,
        },
    },
    quest::{IsQuest, QuestError},
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnwrapPacket;

impl IsQuest for UnwrapPacket {
    fn get_description(&self) -> &'static str {
        "The packet is lying on the ground. Take a look"
    }

    fn list_necessary_collected_items(&self) -> Vec<CollectableItem> {
        Vec::new()
    }

    fn list_necessary_stationed_items(&self) -> Vec<StationaryItem> {
        vec![StationaryItem::Packet(Packet::initiate())]
    }

    fn list_reward_collectable_items(&self) -> Vec<CollectableItem> {
        vec![CollectableItem::RustyKey(RustyKey)]
    }

    fn list_reward_stationary_items(&self) -> Vec<StationaryItem> {
        vec![StationaryItem::TreasureChest(TreasureChest::initiate())]
    }

    fn check_if_solvable(
        &self,
        collected_items: &CollectedItems,
        stationed_items: &StationedItems,
    ) -> Result<RewardItems, QuestError> {
        let mut reward_items = RewardItems::default();

        // check that all collectable items necessary to solve quest are available
        if !self
            .list_necessary_collected_items()
            .iter()
            .all(|necessary_item| collected_items.list.contains(necessary_item))
        {
            return Err(QuestError::CollectableItemMissing);
        }

        // check that all stationed items necessary to solve quest are available
        if !self
            .list_necessary_stationed_items()
            .iter()
            .all(|necessary_item| stationed_items.list.contains(necessary_item))
        {
            return Err(QuestError::StationaryItemMissing);
        }
        // removed used items
        // for used_item in self.list_necessary_collected_items() {
        //     let index = collected_items
        //         .list
        //         .iter()
        //         .position(|available_item| *available_item == used_item)
        //         .expect("Checked before that backpack contains all necessary items");
        //     collected_items.list.remove(index);
        // }
        reward_items.removed_collectable = self.list_necessary_collected_items();
        reward_items.removed_stationed = self.list_necessary_stationed_items();

        // quest is finished
        // self.status = QuestStatus::Finished;

        // generate rewarded items
        // collected_items
        //     .list
        //     .append(&mut self.list_reward_collectable_items());
        reward_items.added_collectable = self.list_reward_collectable_items();
        reward_items.added_stationed = self.list_reward_stationary_items();

        // return new stationed items
        // vec![StationaryItem::TreasureChest(TreasureChest::initiate())]
        Ok(reward_items)
    }
}
