use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{CollectableItem, CollectedItems, branch::Branch, oiled_branch::OiledBranch},
    interaction::{Interactable, InteractionStatus},
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OilDrumInteraction {
    DipBranch(DipBranch),
}

impl OilDrumInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            OilDrumInteraction::DipBranch(inner) => inner,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DipBranch;
impl Interactable for DipBranch {
    fn get_prompt(&self) -> String {
        String::from("Dip branch into oil.")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                // check that branch is available
                if !collected_items
                    .list
                    .contains(&CollectableItem::Branch(Branch))
                {
                    return String::from("You have no branch available.");
                }

                // limit number of oiled branches player can have
                if collected_items
                    .list
                    .iter()
                    .filter(|item| matches!(item, CollectableItem::OiledBranch(..)))
                    .count()
                    >= 2
                {
                    return String::from("You already got enough oiled branches.");
                }

                // remove branch
                let index = collected_items
                    .list
                    .iter()
                    .position(|available_item| *available_item == CollectableItem::Branch(Branch))
                    .expect("Checked before that item is available");
                collected_items.list.remove(index);

                collected_items
                    .list
                    .push(CollectableItem::OiledBranch(OiledBranch));

                String::from("The branch is covered with oil.")
            }
            InteractionStatus::Finished => String::from("No more oil available."),
        }
    }
}
