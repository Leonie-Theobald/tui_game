pub mod blocked_room;
pub mod interaction;
pub mod lost_chest_key;
pub mod packet;
pub mod treasure_chest;

use std::io::{Write, stdout};

use termion::raw::IntoRawMode;

use crate::{
    item::{
        collected_item::CollectedItems,
        stationed_item::{
            blocked_room::BlockedRoom,
            interaction::{Interaction, InteractionStatus},
            lost_chest_key::LostChestKey,
            packet::Packet,
            treasure_chest::TreasureChest,
        },
    },
    player::Player,
    tui::{
        Drawable, Vector,
        pop_up::{InfoPopUp, PopUp},
    },
};

#[macro_export]
macro_rules! method_on_inner_stationed_item {
    // call without parameters
    ($value:expr, $method:ident) => {
        match &$value {
            StationaryItem::Packet(inner) => inner.$method(),
            StationaryItem::TreasureChest(inner) => inner.$method(),
            StationaryItem::BlockedRoom(inner) => inner.$method(),
            StationaryItem::LostChestKey(inner) => inner.$method(),
        }
    };
    // call with parameters
    ($value:expr, $method:ident ($ ( $parameter:expr ),* $(,)? )) => {
        match &$value {
            StationaryItem::Packet(inner) => inner.$method( $( $parameter ),* ),
            StationaryItem::TreasureChest(inner) => inner.$method( $( $parameter ),*),
            StationaryItem::BlockedRoom(inner) => inner.$method( $( $parameter ),* ),
            StationaryItem::LostChestKey(inner) => inner.$method( $( $parameter ),* ),
        }
    };
}

macro_rules! mut_method_on_inner_stationed_item {
    // call without parameters
    ($value:expr, $method:ident) => {
        match &mut $value {
            StationaryItem::Packet(inner) => inner.$method(),
            StationaryItem::TreasureChest(inner) => inner.$method(),
            StationaryItem::BlockedRoom(inner) => inner.$method(),
            StationaryItem::LostChestKey(inner) => inner.$method(),
        }
    };
    // call with parameters
    ($value:expr, $method:ident ($ ( $parameter:expr ),* $(,)? )) => {
        match &mut $value {
            StationaryItem::Packet(inner) => inner.$method( $( $parameter ),* ),
            StationaryItem::TreasureChest(inner) => inner.$method( $( $parameter ),*),
            StationaryItem::BlockedRoom(inner) => inner.$method( $( $parameter ),* ),
            StationaryItem::LostChestKey(inner) => inner.$method( $( $parameter ),* ),
        }
    };
}

#[derive(Debug, PartialEq)]
pub struct StationedItems {
    pub list: Vec<StationaryItem>,
}

impl StationedItems {
    pub fn initiate() -> Self {
        Self {
            list: vec![
                StationaryItem::Packet(Packet::initiate()),
                StationaryItem::BlockedRoom(BlockedRoom::initiate()),
            ],
        }
    }

    pub fn accessible_stationary_item_mut(
        &mut self,
        target_coordinates: &Vector,
    ) -> Option<&mut StationaryItem> {
        // go through each item and check whether any of its covered coordinates is the one
        // the player accesses right now
        for stationed_item in &mut self.list {
            if method_on_inner_stationed_item!(stationed_item, list_blocking_coordinates)
                .contains(target_coordinates)
            {
                // this stationed item is also placed at the coordinates that the player can access
                return Some(stationed_item);
            }
        }

        None
    }

    pub fn accessible_stationary_item(
        &self,
        target_coordinates: &Vector,
    ) -> Option<&StationaryItem> {
        // go through each item and check whether any of its covered coordinates is the one
        // the player accesses right now
        for stationed_item in &self.list {
            if method_on_inner_stationed_item!(stationed_item, list_blocking_coordinates)
                .contains(target_coordinates)
            {
                // this stationed item is also placed at the coordinates that the player can access
                return Some(stationed_item);
            }
        }

        None
    }

    pub fn count_possible_interactions(&self, target_coordinates: &Vector) -> usize {
        match self.accessible_stationary_item(target_coordinates) {
            None => 0,
            Some(item) => method_on_inner_stationed_item!(item, get_linked_interactions).len(),
        }
    }

    pub fn interact(&mut self, player: &Player, collected_items: &mut CollectedItems) {
        // find accessible item
        let target_coordinates = player.get_facing_coordinates();
        let mut accessible_item = match self.accessible_stationary_item_mut(&target_coordinates) {
            None => {
                InfoPopUp {
                    message: String::from("No items available for interaction."),
                }
                .show_to_user();
                return;
            }
            Some(item) => item,
        };

        // prepare terminal
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        // get interactions linked to the selected item
        mut_method_on_inner_stationed_item!(accessible_item, interact(collected_items));
    }
}

pub trait Stationary {
    fn list_blocking_coordinates(&self) -> Vec<Vector>;
    fn get_linked_interactions(&self) -> &Vec<Interaction>;
    fn interact(&mut self, collected_items: &mut CollectedItems)
    where
        Self: std::marker::Sized,
    {
        let prompts = self
            .get_linked_interactions()
            .iter()
            .map(|interaction| interaction.get_inner().get_prompt())
            .collect();
        let pop_up = PopUp { options: prompts };
        let selected_interaction = match pop_up.get_user_selection() {
            Ok(selected_prompt) => self
                .get_linked_interactions()
                .clone()
                .into_iter()
                .filter(|interaction| interaction.get_inner().get_prompt() == selected_prompt)
                .collect::<Vec<_>>()
                .first()
                .expect("Just had the user selection")
                .clone(),
            Err(_err) => return,
        };

        let info_prompt = selected_interaction
            .get_inner()
            .interact(self, collected_items);
        InfoPopUp {
            message: info_prompt,
        }
        .show_to_user();
    }
    fn get_interaction_status(&self) -> InteractionStatus;
    fn set_interaction_status_to_finished(&mut self);
}

#[derive(Clone, Debug, PartialEq)]
pub enum StationaryItem {
    Packet(Packet),
    TreasureChest(TreasureChest),
    BlockedRoom(BlockedRoom),
    LostChestKey(LostChestKey),
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
