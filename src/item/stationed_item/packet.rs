use crate::{
    item::{
        Item,
        stationed_item::{
            Stationary,
            interaction::{Interaction, InteractionStatus, OpenPacket, ShakePacket, StompPacket},
        },
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 15,
    vertical: 7,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    opened: bool,
    linked_interactions: Vec<Interaction>,
}

impl Packet {
    pub fn initiate() -> Self {
        let linked_interactions = vec![
            Interaction::OpenPacket(OpenPacket {}),
            Interaction::StompPacket(StompPacket {}),
            Interaction::ShakePacket(ShakePacket {}),
        ];

        Self {
            opened: false,
            linked_interactions,
        }
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

impl Drawable for Packet {
    fn draw(&self) -> String {
        let symbol = if self.opened { "X" } else { "📦" };

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
