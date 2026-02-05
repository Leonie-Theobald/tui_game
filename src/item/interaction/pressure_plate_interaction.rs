use serde::{Deserialize, Serialize};

use crate::{
    final_cut_scene::play_final_cut_scene,
    item::{
        collected_item::CollectedItems,
        interaction::{Interactable, InteractionStatus},
        stationed_item::Stationary,
    },
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

                play_final_cut_scene();
                unreachable!()
            }
            InteractionStatus::Finished => String::from("Nothing can be done here."),
        }
    }
}
