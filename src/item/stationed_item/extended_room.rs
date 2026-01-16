use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{Interaction, InteractionStatus},
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 10,
    vertical: 7,
};

const DIMENSIONS: Vector = Vector {
    horizontal: 35,
    vertical: 9,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedRoom {
    linked_interactions: Vec<Interaction>,
}

impl ExtendedRoom {
    pub fn initiate() -> Self {
        Self {
            // initiated with empty vector because method for returning interactions wants
            // reference and you cannot return reference to temporary value
            linked_interactions: Vec::new(),
        }
    }
}

impl Item for ExtendedRoom {
    fn get_name(&self) -> &'static str {
        "Room"
    }

    fn get_description(&self) -> &'static str {
        "This room got a fire place."
    }

    fn get_detail_view(&self) -> String {
        String::new()
    }
}

impl Stationary for ExtendedRoom {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinates = Vec::new();

        // top and bottom wall
        for horizontal_offset in 0..=DIMENSIONS.horizontal {
            // leave some space where the door is
            if !(11..=13).contains(&horizontal_offset) && !(22..=24).contains(&horizontal_offset) {
                blocked_coordinates.push(Vector {
                    horizontal: COORDINATES.horizontal + horizontal_offset,
                    vertical: COORDINATES.vertical,
                });
            }

            blocked_coordinates.push(Vector {
                horizontal: COORDINATES.horizontal + horizontal_offset,
                vertical: COORDINATES.vertical + DIMENSIONS.vertical,
            });
        }

        // left and right wall
        for vertical_offset in 0..=DIMENSIONS.vertical {
            blocked_coordinates.push(Vector {
                horizontal: COORDINATES.horizontal,
                vertical: COORDINATES.vertical + vertical_offset,
            });

            blocked_coordinates.push(Vector {
                horizontal: COORDINATES.horizontal + DIMENSIONS.horizontal - 1,
                vertical: COORDINATES.vertical + vertical_offset,
            });
        }

        blocked_coordinates
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        InteractionStatus::Finished
    }

    fn set_interaction_status_to_finished(&mut self) {}
}

impl Drawable for ExtendedRoom {
    fn draw(&self) -> String {
        let mut walls = String::new();

        // top wall
        walls.push_str(&format!(
            "{}├──────────{}─────┴──{}─────────┐",
            // go to start of wall
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16,
            ),
            // leave space for door
            termion::cursor::Goto(
                (COORDINATES.horizontal + 15) as u16,
                (COORDINATES.vertical + 1) as u16,
            ),
            // leave space for second
            termion::cursor::Goto(
                (COORDINATES.horizontal + 26) as u16,
                (COORDINATES.vertical + 1) as u16,
            ),
        ));

        // left and right wall
        for vertical_offset in 1..DIMENSIONS.vertical {
            walls.push_str(&format!(
                "{}│{}│",
                termion::cursor::Goto(
                    (COORDINATES.horizontal + 1) as u16,
                    (COORDINATES.vertical + vertical_offset + 1) as u16,
                ),
                termion::cursor::Goto(
                    (COORDINATES.horizontal + DIMENSIONS.horizontal) as u16,
                    (COORDINATES.vertical + vertical_offset + 1) as u16,
                )
            ))
        }

        // bottom wall
        walls.push_str(&format!(
            "{}└─────────────────────────────────┘",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + DIMENSIONS.vertical + 1) as u16,
            ),
        ));

        walls
    }
}
