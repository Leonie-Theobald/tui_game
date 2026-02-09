use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{Interaction, InteractionStatus},
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector, background::BACKGROUND_COLOR},
};

const COORDINATES: Vector = Vector {
    horizontal: 10,
    vertical: 7,
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

    fn get_room_setup(&self) -> &'static str {
        "├──────────   ─────┴     ─────────┐
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
│                                 │
└─────────────────────────────────┘"
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
        let room = self.get_room_setup();

        let mut blocked_coordinate = Vec::new();
        for (vertical_offset, line) in room.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    _ => blocked_coordinate.push(Vector {
                        horizontal: COORDINATES.horizontal + horizontal_offset,
                        vertical: COORDINATES.vertical + vertical_offset,
                    }),
                }
            }
        }

        blocked_coordinate
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
        let room_setup = self.get_room_setup();

        let mut room = String::new();
        for (vertical_offset, line) in room_setup.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    _ => room.push_str(&format!(
                        "{}{}{}{}",
                        termion::cursor::Goto(
                            (COORDINATES.horizontal + horizontal_offset + 1) as u16,
                            (COORDINATES.vertical + vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(BACKGROUND_COLOR),
                        char,
                        termion::color::Bg(termion::color::Reset),
                    )),
                }
            }
        }

        room
    }
}
