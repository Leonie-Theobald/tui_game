use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{CollectableItem, CollectedItems, marshmallow::Marshmallow, skewer::Skewer},
    interaction::{Interactable, InteractionStatus},
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DroppedMarshmallowInteraction {
    SkewerMarshmallow(SkewerMarshmallow),
}

impl DroppedMarshmallowInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            DroppedMarshmallowInteraction::SkewerMarshmallow(inner) => inner,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkewerMarshmallow;
impl Interactable for SkewerMarshmallow {
    fn get_prompt(&self) -> String {
        String::from("Skewer the Marshmallow")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                if !collected_items
                    .list
                    .contains(&CollectableItem::Skewer(Skewer))
                {
                    return String::from("You have no skewer for pick up.");
                }

                // remove skewer
                let index = collected_items
                    .list
                    .iter()
                    .position(|available_item| *available_item == CollectableItem::Skewer(Skewer))
                    .expect("Checked before that item is available");
                collected_items.list.remove(index);

                collected_items
                    .list
                    .push(CollectableItem::Marshmallow(Marshmallow));
                linked_stationary_item.set_interaction_status_to_finished();
                String::from("You picked up the marshmallow.")
            }
            InteractionStatus::Finished => String::from("Marshmallow is already picked up"),
        }
    }
}
