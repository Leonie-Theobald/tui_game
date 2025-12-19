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

macro_rules! method_on_inner_stationed_item {
    ($value:expr, $method:ident) => {
        match $value {
            StationaryItem::Packet(inner) => inner.$method(),
        }
    };
}

impl Drawable for StationedItems {
    fn draw(&self) -> String {
        let mut joined_items = String::new();

        for item in &self.list {
            let drawn_item = method_on_inner_stationed_item!(item, draw);
            joined_items.push_str(&drawn_item);
        }

        joined_items
    }
}
