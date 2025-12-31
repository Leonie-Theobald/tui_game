use inquire::InquireError;
use inquire_derive::Selectable;

use crate::{
    item::{Item, stationed_item::Stationary},
    quest::{Quest, unwrap_packet::UnwrapPacket},
    tui::{Drawable, Vector, pop_up::InfoPopUp},
};

const COORDINATES: Vector = Vector {
    horizontal: 15,
    vertical: 7,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    linked_quest: Quest,
}

#[derive(Debug, Copy, Clone, Selectable, PartialEq)]
enum PacketInteraction {
    Shake,
    Stomp,
    Open,
}

impl std::fmt::Display for PacketInteraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let interaction = match self {
            PacketInteraction::Shake => "Shake the packet.",
            PacketInteraction::Stomp => "Stomp on the packet.",
            PacketInteraction::Open => "Open the packet.",
        };

        write!(f, "{}", interaction)
    }
}

impl Packet {
    pub fn initiate() -> Self {
        let linked_quest = Quest::UnwrapPacket(UnwrapPacket);
        Self { linked_quest }
    }

    pub fn try_interaction(&mut self) {
        let selected_interaction =
            match PacketInteraction::select("What do you want to try?").prompt() {
                Ok(selection) => selection,
                Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                    return;
                }
                Err(err) => panic!("Error shouldn't occur: {:?}", err),
            };

        let info_prompt = match selected_interaction {
            PacketInteraction::Shake => "Something is rattling in the box.",
            PacketInteraction::Stomp => "Now the packet is flat. Well done...",
            PacketInteraction::Open => "Hurray! You found something.",
        };

        InfoPopUp {
            message: String::from(info_prompt),
        }
        .show_to_user();
    }
}

impl Item for Packet {
    fn get_name(&self) -> &'static str {
        "Wrapped Packet"
    }

    fn get_description(&self) -> &'static str {
        "A small packet wrapped in paper lying on the ground. Take a look inside!"
    }
}

impl Stationary for Packet {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinate = Vec::new();

        // +-1 around the symbol is also blocked
        let horizontal_range = COORDINATES.horizontal..COORDINATES.horizontal + 2;
        for horizontal_index in horizontal_range {
            let vertical_range = COORDINATES.vertical..COORDINATES.vertical + 1;
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

impl Drawable for Packet {
    fn draw(&self) -> String {
        let symbol = "📦";

        format!(
            "{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            symbol,
        )
    }
}
