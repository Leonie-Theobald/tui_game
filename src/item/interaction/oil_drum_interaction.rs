use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{
        CollectableItem, CollectedItems, branch::Branch, marshmallow::Marshmallow,
        oiled_branch::OiledBranch, roasted_marshmallow::RoastedMarshmallow, torch::Torch,
    },
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoastMarshmallow;
impl Interactable for RoastMarshmallow {
    fn get_prompt(&self) -> String {
        String::from("Roast Marshmallow")
    }

    fn interact(
        &self,
        _linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        if !collected_items
            .list
            .contains(&CollectableItem::Marshmallow(Marshmallow))
        {
            return String::from("You have no marshmallow to roast.");
        }

        // remove marshmallow
        let index = collected_items
            .list
            .iter()
            .position(|available_item| *available_item == CollectableItem::Marshmallow(Marshmallow))
            .expect("Checked before that item is available");
        collected_items.list.remove(index);

        collected_items
            .list
            .push(CollectableItem::RoastedMarshmallow(RoastedMarshmallow));

        String::from("You got yourself a crispy marshmallow. Enjoy!")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightBranch;
impl Interactable for LightBranch {
    fn get_prompt(&self) -> String {
        String::from("Light a Branch.")
    }

    fn interact(
        &self,
        _linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        let branch_available = collected_items
            .list
            .contains(&CollectableItem::Branch(Branch));
        let oiled_branch_available = collected_items
            .list
            .contains(&CollectableItem::OiledBranch(OiledBranch));
        match (branch_available, oiled_branch_available) {
            (false, false) => String::from("You have no branch available"),
            (true, false) => String::from("The branch doesn't seem to start burning."),
            (_, true) => {
                // remove oiled branch
                let index = collected_items
                    .list
                    .iter()
                    .position(|available_item| {
                        *available_item == CollectableItem::OiledBranch(OiledBranch)
                    })
                    .expect("Checked before that item is available");
                collected_items.list.remove(index);

                collected_items.list.push(CollectableItem::Torch(Torch));

                String::from("Nice! You build a torch.")
            }
        }
    }
}
