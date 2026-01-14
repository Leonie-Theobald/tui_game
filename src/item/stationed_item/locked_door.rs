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
    horizontal: 21,
    vertical: 7,
};
const WIDTH: usize = 3;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LockedDoor {
    linked_interactions: Vec<Interaction>,
    pub opened: bool,
}

impl LockedDoor {
    pub fn initiate() -> Self {
        let linked_interactions = vec![
            Interaction::LockedDoor(LockedDoorInteraction::Bang(Bang {})),
            Interaction::LockedDoor(LockedDoorInteraction::BreakDown(BreakDown {})),
            Interaction::LockedDoor(LockedDoorInteraction::UnlockWithKey(UnlockWithKey {})),
        ];

        Self {
            linked_interactions,
            opened: false,
        }
    }
}

impl Item for LockedDoor {
    fn get_name(&self) -> &'static str {
        "Locked Door"
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

impl Stationary for LockedDoor {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        if self.opened {
            return Vec::new();
        }

        let mut blocked_coordinate = Vec::new();

        for horizontal_index in 0..WIDTH {
            blocked_coordinate.push(Vector {
                horizontal: COORDINATES.horizontal + horizontal_index,
                vertical: COORDINATES.vertical,
            })
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

impl Drawable for LockedDoor {
    fn draw(&self) -> String {
        if self.opened {
            return String::new();
        }
        let door_string = "█".repeat(WIDTH);

        format!(
            "{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            door_string,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LockedDoorInteraction {
    Bang(Bang),
    BreakDown(BreakDown),
    UnlockWithKey(UnlockWithKey),
}

impl LockedDoorInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            LockedDoorInteraction::Bang(bang) => bang,
            LockedDoorInteraction::BreakDown(break_down) => break_down,
            LockedDoorInteraction::UnlockWithKey(unlock_with_key) => unlock_with_key,
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
