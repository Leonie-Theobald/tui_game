mod item;
mod player;
mod quest;
mod tui;

use std::io::{Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use tui::Drawable;

use crate::{
    item::{
        collected_item::CollectedItems,
        stationed_item::{Stationary, StationaryItem, StationedItems},
    },
    player::Player,
    quest::{IsQuest, Quest},
    tui::{
        Orientation,
        grid::Grid,
        pop_up::{Button, PopUp},
    },
};

fn draw_game(
    grid: &Grid,
    my_player: &Player,
    my_collected_items: &CollectedItems,
    my_stationed_items: &StationedItems,
) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}{}{}{}",
        termion::clear::All,
        grid.draw(),
        my_player.draw(),
        my_collected_items.draw(),
        my_stationed_items.draw(),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}

fn main() {
    let grid = Grid::initiate();
    let mut player = Player::initiate();
    let mut collected_items = CollectedItems::initiate();
    let mut stationed_items = StationedItems::initiate();

    draw_game(&grid, &player, &collected_items, &stationed_items);

    let mut stdout = stdout().into_raw_mode().unwrap();
    for k in stdin().keys() {
        draw_game(&grid, &player, &collected_items, &stationed_items);
        match k.as_ref().unwrap() {
            Key::Char('a') => player.move_in_direction(&grid, &stationed_items, Orientation::West),
            Key::Char('w') => player.move_in_direction(&grid, &stationed_items, Orientation::North),
            Key::Char('d') => player.move_in_direction(&grid, &stationed_items, Orientation::East),
            Key::Char('s') => player.move_in_direction(&grid, &stationed_items, Orientation::South),
            Key::Char('e') => {
                let mut popup_collected_item = PopUp {
                    options: &collected_items.list,
                    active_option_index: 0,
                    buttons: vec![Button::Confirm],
                    active_button_index: 0,
                };
                #[expect(
                    unused_must_use,
                    reason = "We don't need user's selection here. 
                    It is just the API to present the items to the user."
                )]
                popup_collected_item.get_user_selection();
            }
            Key::Char('f') => {
                let target_coordinates = player.get_facing_coordinates();

                // get user's input on what item should be chosen
                let selected_stationary_item = match stationed_items
                    .user_selection_for_accessible_stationary_items(&target_coordinates)
                {
                    Ok(item) => item,
                    Err(_err) => {
                        // return to normal game grid
                        draw_game(&grid, &player, &collected_items, &stationed_items);
                        // write!(
                        //     stdout,
                        //     "{}{}{}{}{}{}",
                        //     termion::clear::All,
                        //     grid.draw(),
                        //     my_player.draw(),
                        //     my_collected_items.draw(),
                        //     my_stationed_items.draw(),
                        //     termion::cursor::Hide
                        // )
                        // .unwrap();
                        // stdout.flush().unwrap();
                        continue;
                    }
                };

                // get quest linked to the selected item
                let linked_quest =
                    method_on_inner_stationed_item!(selected_stationary_item, get_linked_quest);
                let result = match linked_quest {
                    Quest::UnwrapPacket(inner) => {
                        inner.check_if_solvable(&collected_items, &stationed_items)
                    }
                    Quest::UnlockTreasureChest(inner) => {
                        inner.check_if_solvable(&collected_items, &stationed_items)
                    }
                };
                match result {
                    Err(_err) => {
                        draw_game(&grid, &player, &collected_items, &stationed_items);
                        // write!(
                        //     stdout,
                        //     "{}{}{}{}{}{}",
                        //     termion::clear::All,
                        //     grid.draw(),
                        //     my_player.draw(),
                        //     my_collected_items.draw(),
                        //     my_stationed_items.draw(),
                        //     termion::cursor::Hide
                        // )
                        // .unwrap();
                        // stdout.flush().unwrap();
                        continue;
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

                        // remove the stationed item which quests was sovled
                        // mut_method_on_inner_stationed_item!(
                        //     selected_stationary_item,
                        //     change_status_to_not_available
                        // );
                        for used_item in reward_items.removed_stationed {
                            let index = stationed_items
                                .list
                                .iter()
                                .position(|available_item| *available_item == used_item)
                                .expect("Checked before that quest is solvable");
                            stationed_items.list.remove(index);
                        }

                        // add new collectable items
                        collected_items
                            .list
                            .append(&mut reward_items.added_collectable);

                        // add new stationed items
                        stationed_items
                            .list
                            .append(&mut reward_items.added_stationed);
                    }
                }
            }
            Key::Ctrl('c') => break,
            _ => {
                write!(stdout, "\r\nUnknown character... Exiting\r\n").unwrap();
                break;
            }
        };

        draw_game(&grid, &player, &collected_items, &stationed_items);
    }
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )
    .unwrap();
    stdout.suspend_raw_mode().unwrap();
}
