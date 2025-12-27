use crate::{
    item::{Item, stationed_item::Stationary},
    quest::{Quest, open_door::OpenDoor},
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 32,
    vertical: 12,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BlockedRoom {
    linked_quest: Quest,
    dimensions: Vector,
}

impl BlockedRoom {
    pub fn initiate() -> Self {
        let linked_quest = Quest::OpenDoor(OpenDoor);
        let dimensions = Vector {
            horizontal: 29,
            vertical: 10,
        };
        Self {
            linked_quest,
            dimensions,
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

    fn get_linked_quest(&self) -> &Quest {
        &self.linked_quest
    }
}

impl Drawable for BlockedRoom {
    fn draw(&self) -> String {
        let one_line = "█".repeat(self.dimensions.horizontal);

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
