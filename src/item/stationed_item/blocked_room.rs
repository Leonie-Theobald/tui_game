use inquire::InquireError;
use inquire_derive::Selectable;

use crate::{
    item::{
        Item,
        collected_item::{CollectableItem, CollectedItems, rusty_key::RustyKey},
        stationed_item::Stationary,
    },
    quest::{Quest, open_door::OpenDoor},
    tui::{Drawable, Vector, pop_up::InfoPopUp},
};

const COORDINATES: Vector = Vector {
    horizontal: 32,
    vertical: 12,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BlockedRoom {
    linked_quest: Quest,
    opened: bool,
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

#[derive(Debug, Copy, Clone, Selectable, PartialEq)]
pub enum BlockedRoomInteraction {
    BangDoor,
    UnlockWithKey,
    BreakDownDoor,
}

impl std::fmt::Display for BlockedRoomInteraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let interaction = match self {
            BlockedRoomInteraction::BangDoor => "Loudly bang on door.",
            BlockedRoomInteraction::UnlockWithKey => "Use key to open door.",
            BlockedRoomInteraction::BreakDownDoor => "Break down door with raw force",
        };

        write!(f, "{}", interaction)
    }
}

impl Stationary for BlockedRoom {
    fn interact(&mut self, collected_items: &mut CollectedItems) {
        let selected_interaction =
            match BlockedRoomInteraction::select("What do you want to try?").prompt() {
                Ok(selection) => selection,
                Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                    return;
                }
                Err(err) => panic!("Error shouldn't occur: {:?}", err),
            };

        let info_prompt = match selected_interaction {
            BlockedRoomInteraction::BangDoor => "Nobody opens.",
            BlockedRoomInteraction::BreakDownDoor => "This is a massive steel door. No chance.",
            BlockedRoomInteraction::UnlockWithKey => {
                if self.opened {
                    "Door is already open. Please enter."
                } else if !collected_items
                    .list
                    .contains(&CollectableItem::RustyKey(RustyKey))
                {
                    "You have no key to open the door."
                } else {
                    // remove key
                    let index = collected_items
                        .list
                        .iter()
                        .position(|available_item| {
                            *available_item == CollectableItem::RustyKey(RustyKey)
                        })
                        .expect("Checked before that item is available");
                    collected_items.list.remove(index);

                    self.opened = true;

                    "This was the correct key to use."
                }
            }
        };

        InfoPopUp {
            message: String::from(info_prompt),
        }
        .show_to_user();
    }
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

    fn get_linked_quest(&self) -> &Quest {
        &self.linked_quest
    }

    fn get_interaction_status(&self) -> super::interaction::InteractionStatus {
        todo!()
    }

    fn set_interaction_status_to_finished(&mut self) {
        todo!()
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
