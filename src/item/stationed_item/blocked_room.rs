use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        collected_item::{CollectableItem, CollectedItems, rusty_key::RustyKey},
        interaction::{Interactable, Interaction, InteractionStatus},
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 32,
    vertical: 12,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockedRoom {
    linked_interactions: Vec<Interaction>,
    pub opened: bool,
    dimensions: Vector,
}

impl BlockedRoom {
    pub fn initiate() -> Self {
        let dimensions = Vector {
            horizontal: 29,
            vertical: 8,
        };
        let linked_interactions = vec![
            Interaction::Door(DoorInteraction::Bang(Bang {})),
            Interaction::Door(DoorInteraction::BreakDown(BreakDown {})),
            Interaction::Door(DoorInteraction::UnlockWithKey(UnlockWithKey {})),
        ];

        Self {
            linked_interactions,
            dimensions,
            opened: false,
        }
    }
}

impl Item for BlockedRoom {
    fn get_name(&self) -> &'static str {
        "Locked Room"
    }

    fn get_description(&self) -> &'static str {
        "This door is locked. Try again with a key."
    }

    fn get_detail_view(&self) -> String {
        String::from(
            "                                                                               
                                                                               
                                                                               
                                                                               
                   █▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█                                                       
                  █           |            █                                
                  █           |            █                                      
                  █           |            █                                      
                █▛▀           |            ▀█▌                                
                █▌            |             █▌                                
                █▌            |             █▌                                
                █▌            |             █▌                                
                █▌          O | O           █▌                                
                █▌            |             █▌                                
                █▌            |             █▌                                
                █▌            |             █▌                                
                █▌            |             █▌                                
                █▌            |             █▌                                
    //////////////////////////////////////////////////////////////////          
    //////////////////////////////////////////////////////////////////          
                                                                                
                                                                                ",
        )
    }
}

impl Stationary for BlockedRoom {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        if self.opened {
            return Vec::new();
        }

        let mut blocked_coordinate = Vec::new();

        let horizontal_range =
            COORDINATES.horizontal..COORDINATES.horizontal + self.dimensions.horizontal;
        for horizontal_index in horizontal_range {
            let vertical_range =
                COORDINATES.vertical..COORDINATES.vertical + self.dimensions.vertical;
            for vertical_index in vertical_range {
                blocked_coordinate.push(Vector {
                    horizontal: horizontal_index,
                    vertical: vertical_index,
                })
            }
        }

        blocked_coordinate
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        if self.opened {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.opened = true;
    }
}

impl Drawable for BlockedRoom {
    fn draw(&self) -> String {
        if self.opened {
            return String::new();
        }
        let symbol = "█";

        let one_line = symbol.repeat(self.dimensions.horizontal);

        let mut whole_string = String::new();
        for index in 0..self.dimensions.vertical {
            whole_string.push_str(&format!(
                "{}{}",
                termion::cursor::Goto(
                    (COORDINATES.horizontal + 1) as u16,
                    (COORDINATES.vertical + index + 1) as u16
                ),
                one_line,
            ))
        }

        whole_string
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DoorInteraction {
    Bang(Bang),
    BreakDown(BreakDown),
    UnlockWithKey(UnlockWithKey),
}

impl DoorInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            DoorInteraction::Bang(bang) => bang,
            DoorInteraction::BreakDown(break_down) => break_down,
            DoorInteraction::UnlockWithKey(unlock_with_key) => unlock_with_key,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bang;
impl Interactable for Bang {
    fn get_prompt(&self) -> String {
        String::from("Loudly bang on the door")
    }

    fn interact(
        &self,
        _linked_stationary_item: &mut dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> String {
        String::from("Nobody opens")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BreakDown;
impl Interactable for BreakDown {
    fn get_prompt(&self) -> String {
        String::from("Break down door with raw force")
    }

    fn interact(
        &self,
        _linked_stationary_item: &mut dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> String {
        String::from("This is a massive steel door. No chance")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnlockWithKey;
impl Interactable for UnlockWithKey {
    fn get_prompt(&self) -> String {
        String::from("Use key to open door")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        if !collected_items
            .list
            .contains(&CollectableItem::RustyKey(RustyKey))
        {
            return String::from("You have no key to open the door.");
        }

        // remove key
        let index = collected_items
            .list
            .iter()
            .position(|available_item| *available_item == CollectableItem::RustyKey(RustyKey))
            .expect("Checked before that item is available");
        collected_items.list.remove(index);

        // finish interaction status
        linked_stationary_item.set_interaction_status_to_finished();

        String::from("This was the correct key to use.")
    }
}
