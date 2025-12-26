pub mod packet;
pub mod treasure_chest;

use crate::{
    item::{
        Item,
        collected_item::CollectedItems,
        stationed_item::{packet::Packet, treasure_chest::TreasureChest},
    },
    player::Player,
    quest::{
        IsQuest, Quest, unlock_treasure_chest::UnlockTreasureChest, unwrap_packet::UnwrapPacket,
    },
    tui::{
        Drawable, TuiError, Vector,
        game::draw_game,
        grid::Grid,
        pop_up::{Button, PopUp},
    },
};

#[macro_export]
macro_rules! method_on_inner_stationed_item {
    ($value:expr, $method:ident) => {
        match &$value {
            StationaryItem::Packet(inner) => inner.$method(),
            StationaryItem::TreasureChest(inner) => inner.$method(),
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
            list: vec![StationaryItem::Packet(Packet::initiate())],
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

    pub fn user_selection_for_accessible_stationary_items(
        &self,
        target_coordinates: &Vector,
    ) -> Result<StationaryItem, TuiError> {
        let accessible_items = self.accessible_stationary_items(target_coordinates);
        let descriptions = &accessible_items
            .iter()
            .map(|item| method_on_inner_stationed_item!(item, get_description))
            .collect();
        let mut popup_stationed_item_selection = PopUp {
            options: descriptions,
            active_option_index: 0,
            buttons: vec![Button::Cancel, Button::Confirm],
            active_button_index: 1,
        };

        let selected_index = popup_stationed_item_selection.get_user_selection()?;

        Ok(self
            .list
            .get(selected_index)
            .expect("Just selected this option by popup")
            .clone())
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
                    draw_game(grid, player, collected_items, self);
                    return;
                }
            };

        // get quest linked to the selected item
        let linked_quest =
            method_on_inner_stationed_item!(selected_stationary_item, get_linked_quest);
        let result = match linked_quest {
            Quest::UnwrapPacket(_inner) => UnwrapPacket::check_if_solvable(collected_items, self),
            Quest::UnlockTreasureChest(_inner) => {
                UnlockTreasureChest::check_if_solvable(collected_items, self)
            }
        };
        match result {
            Err(_err) => {
                draw_game(grid, player, collected_items, self);
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum StationaryItem {
    Packet(Packet),
    TreasureChest(TreasureChest),
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
