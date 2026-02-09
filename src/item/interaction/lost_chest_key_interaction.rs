use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{CollectableItem, CollectedItems, chest_key::ChestKey},
    interaction::{Interactable, InteractionStatus},
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LostChestKeyInteraction {
    PickUp(PickUp),
}

impl LostChestKeyInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            LostChestKeyInteraction::PickUp(inner) => inner,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PickUp;
impl Interactable for PickUp {
    fn get_prompt(&self) -> String {
        String::from("Take the lost key")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                collected_items
                    .list
                    .push(CollectableItem::ChestKey(ChestKey));
                linked_stationary_item.set_interaction_status_to_finished();
                String::from("You found a old looking key.")
            }
            InteractionStatus::Finished => String::from("Nothing there"),
        }
    }
}
