use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::CollectedItems,
    interaction::{Interactable, InteractionStatus},
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PressurePlateInteraction {
    Press(Press),
}

impl PressurePlateInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            PressurePlateInteraction::Press(inner) => inner,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Press;
impl Interactable for Press {
    fn get_prompt(&self) -> String {
        String::from("Press hidden pressure plate in wall.")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                linked_stationary_item.set_interaction_status_to_finished();

                String::from("The wall has moved to the side revealing a hidden terminal.")
            }
            InteractionStatus::Finished => String::from("Pressure plate is already activated."),
        }
    }
}
