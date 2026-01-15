use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        collected_item::{CollectableItem, CollectedItems, branch::Branch, skewer::Skewer},
        interaction::{Interactable, Interaction, InteractionStatus},
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 38,
    vertical: 8,
};
const DIMENSIONS: Vector = Vector {
    horizontal: 2,
    vertical: 1,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Firewood {
    linked_interactions: Vec<Interaction>,
}

impl Firewood {
    pub fn initiate() -> Self {
        Self {
            linked_interactions: vec![
                Interaction::Firewood(FirewoodInteraction::TakeBranch(TakeBranch)),
                Interaction::Firewood(FirewoodInteraction::ChopSkewer(ChopSkewer)),
            ],
        }
    }
}

impl Item for Firewood {
    fn get_name(&self) -> &'static str {
        "Firewood"
    }

    fn get_description(&self) -> &'static str {
        "Pile of Wood"
    }

    fn get_detail_view(&self) -> String {
        String::from(
            "                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                     ____ __                                    
                                   /  _  \\  \\_                                  
                                   |  O\\ |    \\_                                
                           ________\\_____/______\\________                       
                          /                       /      \\                      
                         /                       /   __   \\                     
                         |                       |  /  \\  |                     
                         |                       |  \\__/  |                     
                         \\                       \\        /                     
                          \\_______________________\\______/",
        )
    }
}

impl Stationary for Firewood {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinates = Vec::new();
        for horizontal_offset in 0..DIMENSIONS.horizontal {
            for vertical_offset in 0..DIMENSIONS.vertical {
                blocked_coordinates.push(Vector {
                    horizontal: COORDINATES.horizontal + horizontal_offset,
                    vertical: COORDINATES.vertical + vertical_offset,
                });
            }
        }

        blocked_coordinates
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        InteractionStatus::Open
    }

    fn set_interaction_status_to_finished(&mut self) {}
}

impl Drawable for Firewood {
    fn draw(&self) -> String {
        format!(
            "{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            "🪵"
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FirewoodInteraction {
    TakeBranch(TakeBranch),
    ChopSkewer(ChopSkewer),
}

impl FirewoodInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            FirewoodInteraction::TakeBranch(open_packet) => open_packet,
            FirewoodInteraction::ChopSkewer(stomp_packet) => stomp_packet,
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
                collected_items.list.push(CollectableItem::Skewer(Skewer));
                String::from("You got a good skewer.")
            }
            InteractionStatus::Finished => String::from("No more little branches available."),
        }
    }
}
