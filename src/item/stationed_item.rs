pub mod packet;
pub mod treasure_chest;

use crate::{
    item::stationed_item::{packet::Packet, treasure_chest::TreasureChest},
    method_on_inner_stationed_item,
    quest::Quest,
    tui::{Drawable, Vector},
};

#[derive(Debug, PartialEq)]
pub struct StationedItems {
    pub list: Vec<StationaryItem>,
}

impl StationedItems {
    pub fn initiate() -> Self {
        Self {
            list: vec![StationaryItem::Packet(Packet::initiate())],
        }
    }

    pub fn accessible_stationary_items(
        &mut self,
        target_coordinates: Vector,
    ) -> Vec<&mut StationaryItem> {
        // go through each item and check whether any of its covered coordinates is the one
        // the player accesses right now
        let mut accessible_stationed_items = Vec::new();
        for stationed_item in &mut self.list {
            if method_on_inner_stationed_item!(stationed_item, list_blocking_coordinates)
                .contains(&target_coordinates)
            {
                // this stationed item is also placed at the coordinates that the player can access
                accessible_stationed_items.push(stationed_item);
            }
        }

        accessible_stationed_items
    }
}

pub trait Stationary {
    fn list_blocking_coordinates(&self) -> Vec<Vector>;
    fn get_linked_quest(&mut self) -> &mut Quest;
    fn get_status(&self) -> &StationaryStatus;
    fn change_status_to_not_available(&mut self);
}

#[derive(Clone, Debug, PartialEq)]
pub enum StationaryItem {
    Packet(Packet),
    TreasureChest(TreasureChest),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StationaryStatus {
    Visible,
    NotAvailable,
}

#[macro_export]
macro_rules! method_on_inner_stationed_item {
    ($value:expr, $method:ident) => {
        match &$value {
            StationaryItem::Packet(inner) => inner.$method(),
            StationaryItem::TreasureChest(inner) => inner.$method(),
        }
    };
}

#[macro_export]
macro_rules! mut_method_on_inner_stationed_item {
    ($value:expr, $method:ident) => {
        match $value {
            StationaryItem::Packet(inner) => inner.$method(),
            StationaryItem::TreasureChest(inner) => inner.$method(),
        }
    };
}

impl Drawable for StationedItems {
    fn draw(&self) -> String {
        let mut joined_items = String::new();

        for item in &self.list {
            if method_on_inner_stationed_item!(item, get_status) != &StationaryStatus::Visible {
                // only visible items should be drawn
                continue;
            }
            let drawn_item = method_on_inner_stationed_item!(item, draw);
            joined_items.push_str(&drawn_item);
        }

        joined_items
    }
}
