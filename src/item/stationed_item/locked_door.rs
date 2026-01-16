use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            locked_door_interaction::{Bang, BreakDown, LockedDoorInteraction, UnlockWithKey},
        },
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
            Interaction::LockedDoor(LockedDoorInteraction::Bang(Bang)),
            Interaction::LockedDoor(LockedDoorInteraction::BreakDown(BreakDown)),
            Interaction::LockedDoor(LockedDoorInteraction::UnlockWithKey(UnlockWithKey)),
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
