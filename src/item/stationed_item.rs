pub mod packet;

use crate::{
    item::stationed_item::packet::Packet,
    method_on_inner_stationed_item,
    quest_log::quest::Quest,
    tui::{Drawable, Vector},
};

#[derive(PartialEq)]
pub struct StationedItems {
    pub list: Vec<StationaryItem>,
}

impl StationedItems {
    pub fn initiate() -> Self {
        Self {
            list: vec![StationaryItem::Packet(Packet::initiate())],
        }
    }

    pub fn accessible_stationary_items(&self, target_coordinates: Vector) -> Vec<&StationaryItem> {
        // go through each item and check whether any of its covered coordinates is the one
        // the player accesses right now
        let mut accessible_stationed_items = Vec::new();
        for stationed_item in &self.list {
            if method_on_inner_stationed_item!(stationed_item, list_blocking_coordinates)
                .contains(&target_coordinates)
            {
                // this stationed item is also placed at the coordinates that the player can access
                accessible_stationed_items.push(stationed_item);
                break;
            }
        }

        accessible_stationed_items
    }

    // TODO: Changed to Vec<&Quest>
    pub fn get_all_linked_quests(&self) -> Vec<Quest> {
        self.list
            .iter()
            .map(|stationed_item| {
                method_on_inner_stationed_item!(stationed_item, get_linked_quest).clone()
            })
            .collect()
    }
}

pub trait Stationary {
    fn list_blocking_coordinates(&self) -> Vec<Vector>;
    fn get_linked_quest(&self) -> &Quest;
}

#[derive(Clone, Debug, PartialEq)]
pub enum StationaryItem {
    Packet(Packet),
}

#[macro_export]
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
