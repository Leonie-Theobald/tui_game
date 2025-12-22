use crate::{
    item::{
        collected_item::{
            CollectableItem, candle::Candle, rusty_key::RustyKey, treasure_map::TreasureMap,
        },
        stationed_item::StationaryItem,
    },
    quest::{IsQuest, QuestError, QuestStatus},
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnlockTreasureChest {
    pub status: QuestStatus,
}

impl IsQuest for UnlockTreasureChest {
    fn get_description(&self) -> &'static str {
        "The chest is locked. Try to open it."
    }

    fn list_necessary_collected_items(&self) -> Vec<CollectableItem> {
        vec![CollectableItem::RustyKey(RustyKey)]
    }

    fn list_reward_collectable_items(&self) -> Vec<CollectableItem> {
        vec![
            CollectableItem::Candle(Candle),
            CollectableItem::TreasureMap(TreasureMap),
        ]
    }

    fn list_reward_stationary_items(&self) -> Vec<StationaryItem> {
        Vec::new()
    }

    fn try_solve(
        &mut self,
        collected_items: &mut crate::item::collected_item::CollectedItems,
    ) -> Result<(), QuestError> {
        // check that all collectable items necessary to solve quest are available
        if !self
            .list_necessary_collected_items()
            .iter()
            .all(|necessary_item| collected_items.list.contains(necessary_item))
        {
            return Err(QuestError::CollectableItemMissing);
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
        self.status = QuestStatus::Finished;

        // generate rewarded items
        collected_items
            .list
            .append(&mut self.list_reward_collectable_items());

        // return new stationed items
        Ok(())
    }
}
