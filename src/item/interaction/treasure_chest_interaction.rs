use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{
        CollectableItem, CollectedItems, candle::Candle, chest_key::ChestKey,
        treasure_map::TreasureMap,
    },
    interaction::Interactable,
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TreasureChestInteraction {
    Unlock(Unlock),
}

impl TreasureChestInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            TreasureChestInteraction::Unlock(unlock) => unlock,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unlock;
impl Interactable for Unlock {
    fn get_prompt(&self) -> String {
        String::from("Use key to open chest")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        if !collected_items
            .list
            .contains(&CollectableItem::ChestKey(ChestKey))
        {
            return String::from("You have no key to open the treasure chest.");
        }

        // remove key
        let index = collected_items
            .list
            .iter()
            .position(|available_item| *available_item == CollectableItem::ChestKey(ChestKey))
            .expect("Checked before that item is available");
        collected_items.list.remove(index);

        // add collectable items
        collected_items.list.push(CollectableItem::Candle(Candle));
        collected_items
            .list
            .push(CollectableItem::TreasureMap(TreasureMap));

        // finish interaction status
        linked_stationary_item.set_interaction_status_to_finished();

        String::from("This was the correct key to use.")
    }
}
