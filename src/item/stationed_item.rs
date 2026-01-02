pub mod blocked_room;
pub mod lost_chest_key;
pub mod packet;
pub mod treasure_chest;

use std::io::{Write, stdout};

use termion::raw::IntoRawMode;

use crate::{
    item::{
        Item,
        collected_item::CollectedItems,
        stationed_item::{
            blocked_room::BlockedRoom, lost_chest_key::LostChestKey, packet::Packet,
            treasure_chest::TreasureChest,
        },
    },
    method_on_inner_quest,
    player::Player,
    quest::{
        IsQuest, Quest, collect_chest_key::CollectChestKey, open_door::OpenDoor,
        unlock_treasure_chest::UnlockTreasureChest, unwrap_packet::UnwrapPacket,
    },
    tui::{
        Drawable, TuiError, Vector,
        game::draw_game,
        grid::Grid,
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

    pub fn accessible_stationary_items(&self, target_coordinates: &Vector) -> Vec<StationaryItem> {
        // go through each item and check whether any of its covered coordinates is the one
        // the player accesses right now
        let mut accessible_stationed_items = Vec::new();
        for stationed_item in &self.list {
            if method_on_inner_stationed_item!(stationed_item, list_blocking_coordinates)
                .contains(target_coordinates)
            {
                // this stationed item is also placed at the coordinates that the player can access
                accessible_stationed_items.push(stationed_item.clone());
            }
        }

        accessible_stationed_items
    }

    pub fn accessible_stationary_item(
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

    pub fn user_selection_for_accessible_stationary_items(
        &self,
        target_coordinates: &Vector,
    ) -> Result<StationaryItem, TuiError> {
        let accessible_items = self.accessible_stationary_items(target_coordinates);
        let descriptions = &accessible_items
            .iter()
            .map(|item| method_on_inner_stationed_item!(item, get_description))
            .collect::<Vec<_>>();
        let popup_stationed_item_selection = PopUp {
            options: descriptions.clone(),
        };

        let selected_description = popup_stationed_item_selection.get_user_selection()?;
        let selected_item = accessible_items
            .into_iter()
            .filter(|item| {
                let description = method_on_inner_stationed_item!(item, get_description);
                description == selected_description
            })
            .collect::<Vec<_>>()
            .first()
            .expect("User just selected this item based on descriptions")
            .clone();

        Ok(selected_item)
    }

    pub fn interact(&mut self, player: &Player, collected_items: &mut CollectedItems) {
        // find accessible item
        let target_coordinates = player.get_facing_coordinates();
        let mut accessible_item = match self.accessible_stationary_item(&target_coordinates) {
            None => {
                InfoPopUp {
                    message: String::from("No items available for interaction."),
                }
                .show_to_user();
                return;
            }
            Some(item) => item,
        };
        // if accessible_items.len() > 1 {
        //     panic!("There should always be only one Stationary item per pixel.")
        // }

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

        // get user's choice for interaction
        // let mut selected_stationary_item = match accessible_items.first_mut() {
        //     None => {
        //         InfoPopUp {
        //             message: String::from("No items available for interaction."),
        //         }
        //         .show_to_user();
        //         return;
        //     }
        //     Some(item) => item,
        // };

        // get interactions linked to the selected item
        mut_method_on_inner_stationed_item!(accessible_item, interact(collected_items));
    }

    pub fn select_and_solve_linked_quest(
        &mut self,
        grid: &Grid,
        player: &Player,
        collected_items: &mut CollectedItems,
    ) {
        let target_coordinates = player.get_facing_coordinates();

        // get user's input on what item should be chosen
        let selected_stationary_item =
            match self.user_selection_for_accessible_stationary_items(&target_coordinates) {
                Ok(item) => item,
                Err(_err) => {
                    // return to normal game grid
                    draw_game(grid, player, self);
                    return;
                }
            };

        // get quest linked to the selected item
        let linked_quest =
            method_on_inner_stationed_item!(selected_stationary_item, get_linked_quest);
        match method_on_inner_quest!(linked_quest, check_if_solvable(collected_items, self)) {
            Err(_err) => {
                draw_game(grid, player, self);
            }
            Ok(mut reward_items) => {
                // remove collectable items used for this quest
                for used_item in reward_items.removed_collectable {
                    let index = collected_items
                        .list
                        .iter()
                        .position(|available_item| *available_item == used_item)
                        .expect("Checked before that quest is solvable");
                    collected_items.list.remove(index);
                }

                // remove the stationed item which quests was solved
                for used_item in reward_items.removed_stationed {
                    let index = self
                        .list
                        .iter()
                        .position(|available_item| *available_item == used_item)
                        .expect("Checked before that quest is solvable");
                    self.list.remove(index);
                }

                // add new collectable items
                collected_items
                    .list
                    .append(&mut reward_items.added_collectable);

                // add new stationed items
                self.list.append(&mut reward_items.added_stationed);
            }
        }
    }
}

pub trait Stationary {
    fn list_blocking_coordinates(&self) -> Vec<Vector>;
    fn get_linked_quest(&self) -> &Quest;
    fn interact(&mut self, collected_items: &mut CollectedItems);
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
