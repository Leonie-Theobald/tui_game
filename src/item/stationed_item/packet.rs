use crate::{
    item::{
        Item,
        stationed_item::{Stationary, StationaryStatus},
    },
    quest::{Quest, QuestStatus, unwrap_packet::UnwrapPacket},
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 15,
    vertical: 7,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    linked_quest: Quest,
    status: StationaryStatus,
}

impl Packet {
    pub fn initiate() -> Self {
        let linked_quest = Quest::UnwrapPacket(UnwrapPacket {
            status: QuestStatus::Open,
        });
        Self {
            linked_quest,
            status: StationaryStatus::Visible,
        }
    }
}

impl Item for Packet {
    fn get_name(&self) -> &'static str {
        "Wrapped Packet"
    }

    fn get_description(&self) -> &'static str {
        "A small packet wrapped in paper lying on the ground."
    }
}

impl Stationary for Packet {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        if self.status != StationaryStatus::Visible {
            return Vec::new();
        }

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

    fn get_linked_quest(&mut self) -> &mut Quest {
        &mut self.linked_quest
    }

    fn get_status(&self) -> &StationaryStatus {
        &self.status
    }

    fn change_status_to_not_available(&mut self) {
        self.status = StationaryStatus::NotAvailable;
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
