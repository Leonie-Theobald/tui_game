pub mod packet;

use crate::{
    item::stationed_item::packet::Packet,
    tui::{Drawable, Vector},
};

#[derive(PartialEq)]
pub struct StationedItems {
    pub list: Vec<StationaryItem>,
}

impl StationedItems {
    pub fn initiate() -> Self {
        Self {
            list: vec![StationaryItem::Packet(Packet)],
        }
    }
}

pub trait Stationary {
    fn list_blocking_coordinates(&self) -> Vec<Vector>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum StationaryItem {
    Packet(Packet),
}

impl Drawable for StationedItems {
    fn draw(&self) -> String {
        let mut joined_items = String::new();

        for item in &self.list {
            let drawn_item = match item {
                StationaryItem::Packet(packet) => packet.draw(),
            };
            joined_items.push_str(&drawn_item);
        }

        joined_items
    }
}
