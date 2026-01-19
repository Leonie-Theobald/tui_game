use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{CollectableItem, CollectedItems, torch::Torch},
    interaction::{Interactable, InteractionStatus},
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TorchHolderInteraction {
    MountTorch(MountTorch),
}

impl TorchHolderInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            TorchHolderInteraction::MountTorch(inner) => inner,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTorch;
impl Interactable for MountTorch {
    fn get_prompt(&self) -> String {
        String::from("Mount torch in holder.")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                // check that torch is available
                if !collected_items
                    .list
                    .contains(&CollectableItem::Torch(Torch))
                {
                    return String::from("You have no torch available.");
                }

                // remove torch
                let index = collected_items
                    .list
                    .iter()
                    .position(|available_item| *available_item == CollectableItem::Torch(Torch))
                    .expect("Checked before that item is available");
                collected_items.list.remove(index);

                linked_stationary_item.set_interaction_status_to_finished();

                String::from("The torch is mounted and the room is lit now.")
            }
            InteractionStatus::Finished => String::from("The holder already has a torch in it."),
        }
    }
}
