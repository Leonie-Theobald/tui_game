use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        stationed_item::{
            Stationary,
            interaction::{
                BangDoor, BreakDownDoor, Interaction, InteractionStatus, UnlockDoorWithKey,
            },
        },
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
            Interaction::BangDoor(BangDoor {}),
            Interaction::BreakDownDoor(BreakDownDoor {}),
            Interaction::UnlockDoorWithKey(UnlockDoorWithKey {}),
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
