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
    horizontal: 29,
    vertical: 3,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DarkRoom {
    linked_interactions: Vec<Interaction>,
}

impl DarkRoom {
    pub fn initiate() -> Self {
        let linked_interactions = vec![];

        Self {
            linked_interactions,
        }
    }
    fn get_room_setup(&self) -> &'static str {
        "┰──────────────┐
│              │
│               
│              │
┴     ─────────┨"
    }
}

impl Item for DarkRoom {
    fn get_name(&self) -> &'static str {
        "Dark Room"
    }

    fn get_description(&self) -> &'static str {
        "There is not much to see in this room without light."
    }

    fn get_detail_view(&self) -> String {
        String::new()
    }
}

impl Stationary for DarkRoom {
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
        InteractionStatus::Open
    }

    fn set_interaction_status_to_finished(&mut self) {}
}

impl Drawable for DarkRoom {
    fn draw(&self) -> String {
        let room_setup = self.get_room_setup();

        let mut room = String::new();
        for (vertical_offset, line) in room_setup.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    _ => room.push_str(&format!(
                        "{}{}",
                        termion::cursor::Goto(
                            (COORDINATES.horizontal + horizontal_offset + 1) as u16,
                            (COORDINATES.vertical + vertical_offset + 1) as u16
                        ),
                        char,
                    )),
                }
            }
        }

        room
    }
}
