use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{CollectableItem, CollectedItems, branch::Branch, skewer::Skewer},
    interaction::{Interactable, InteractionStatus},
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FirewoodInteraction {
    TakeBranch(TakeBranch),
    ChopSkewer(ChopSkewer),
}

impl FirewoodInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            FirewoodInteraction::TakeBranch(inner) => inner,
            FirewoodInteraction::ChopSkewer(inner) => inner,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TakeBranch;
impl Interactable for TakeBranch {
    fn get_prompt(&self) -> String {
        String::from("Take away a branch.")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                if collected_items
                    .list
                    .iter()
                    .filter(|item| matches!(item, CollectableItem::Branch(..)))
                    .count()
                    >= 2
                {
                    return String::from("You already got enough branches.");
                }
                collected_items.list.push(CollectableItem::Branch(Branch));
                String::from("You got a quite straight branch.")
            }
            InteractionStatus::Finished => String::from("No more branches available."),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChopSkewer;
impl Interactable for ChopSkewer {
    fn get_prompt(&self) -> String {
        String::from("Chop a small skewer.")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                if collected_items
                    .list
                    .contains(&CollectableItem::Skewer(Skewer))
                {
                    return String::from("You already got a skewers.");
                }
                collected_items.list.push(CollectableItem::Skewer(Skewer));
                String::from("You got a good skewer.")
            }
            InteractionStatus::Finished => String::from("No more little branches available."),
        }
    }
}
