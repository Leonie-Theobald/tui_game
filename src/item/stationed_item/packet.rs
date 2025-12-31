use crate::{
    item::{
        Item,
        stationed_item::{Stationary, interaction::Interaction},
    },
    quest::{Quest, unwrap_packet::UnwrapPacket},
    tui::{Drawable, Vector, pop_up::PopUp},
};

const COORDINATES: Vector = Vector {
    horizontal: 15,
    vertical: 7,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    linked_quest: Quest,
    interactions: Vec<Interaction>,
}

impl Packet {
    pub fn initiate() -> Self {
        let linked_quest = Quest::UnwrapPacket(UnwrapPacket);
        let interactions = Vec::new();
        Self {
            linked_quest,
            interactions,
        }
    }

    pub fn try_interaction(&mut self) {
        let options = vec![
            "Shake the packet.",
            "Stomp on the packet.",
            "Open the packet.",
        ];

        let pop_up = PopUp { options };

        let selected_option = match pop_up.get_user_selection() {
            Err(_err) => return,
            Ok(index) => index,
        };

        let info_pop_up = PopUp {
            options: vec![selected_option],
        };
        info_pop_up.get_user_selection().unwrap();
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
