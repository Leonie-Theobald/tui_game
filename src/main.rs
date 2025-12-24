mod item;
mod player;
mod quest;
mod tui;

use std::io::{Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use tui::Drawable;

use crate::{
    item::{
        Item,
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

fn main() {
    println!("INF: Game is initiated");

    let mut my_player = Player::initiate();
    let mut my_collected_items = CollectedItems::initiate();
    let mut my_stationed_items = StationedItems::initiate();
    let grid = Grid::import_from_string(
        "┌──────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│                                                                              │
│         ┌───────────────────────────┬──────────────────────┐                 │
│         │                           │                      │                 │
│         │                           │                      │                 │
│         │                                                  │                 │
│         │                                                  │                 │
│         │                                                  │                 │
│         │                           │                      │                 │
│         │                           │                      │                 │
│         ├──────────   ───────┬──────┴────────   ───────────┤                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         │                                                  │                 │
│         │                                                  │                 │
│         │                                                  │                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         └────────────────────┴─────────────────────────────┘                 │
└──────────────────────────────────────────────────────────────────────────────┘",
    );

    println!("Stationed Items: {:?}", my_stationed_items);
    println!("Collected Items: {:?}", my_collected_items);

    let mut stdout = stdout().into_raw_mode().unwrap();
    for k in stdin().keys() {
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
        match k.as_ref().unwrap() {
            Key::Char('a') => {
                my_player.move_in_direction(&grid, &my_stationed_items, Orientation::West)
            }
            Key::Char('w') => {
                my_player.move_in_direction(&grid, &my_stationed_items, Orientation::North)
            }
            Key::Char('d') => {
                my_player.move_in_direction(&grid, &my_stationed_items, Orientation::East)
            }
            Key::Char('s') => {
                my_player.move_in_direction(&grid, &my_stationed_items, Orientation::South)
            }
            Key::Char('e') => {
                let mut popup_collected_item = PopUp {
                    options: &my_collected_items.list,
                    active_option_index: 0,
                    buttons: vec![Button::Confirm],
                    active_button_index: 0,
                };
                // we do not really need user's selection
                // this is just available through the PopUp API
                #[allow(unused_must_use)]
                popup_collected_item.get_user_selection();
            }
            Key::Char('f') => {
                let target_coordinates = my_player.get_facing_coordinates();
                let mut accessible_items =
                    my_stationed_items.accessible_stationary_items(target_coordinates.clone());
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

                if let Err(_err) = popup_stationed_item_selection.get_user_selection() {
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
                    continue;
                };
                let selected_accesible_item = accessible_items
                    .get_mut(popup_stationed_item_selection.active_option_index)
                    .expect("Just selected this option by popup");
                let linked_quest =
                    mut_method_on_inner_stationed_item!(selected_accesible_item, get_linked_quest);

                let result = match linked_quest {
                    Quest::UnwrapPacket(inner) => inner.try_solve(&mut my_collected_items),
                    Quest::UnlockTreasureChest(inner) => inner.try_solve(&mut my_collected_items),
                };
                if let Err(_err) = result {
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
                    continue;
                }

                // remove used stationary item and add new ones
                let mut new_stationed_items =
                    method_on_inner_quest!(linked_quest, list_reward_stationary_items);
                my_stationed_items.list.append(&mut new_stationed_items);
                let mut accessible_items =
                    my_stationed_items.accessible_stationary_items(target_coordinates);
                match accessible_items.first_mut() {
                    Some(item) => {
                        mut_method_on_inner_stationed_item!(item, change_status_to_not_available)
                    }
                    None => {
                        continue;
                    }
                };
            }
            Key::Ctrl('c') => break,
            _ => {
                write!(stdout, "\r\nUnknown character... Exiting\r\n").unwrap();
                break;
            }
        };

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
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show
    )
    .unwrap();
    stdout.suspend_raw_mode().unwrap();
    println!(
        "stationed_items: {:?}\ncollected_items:{:?}\n",
        my_stationed_items, my_collected_items
    );

    // println!(
    //     "All your quests:\n{}",
    //     my_player.show_all_quests()
    // );
    //
    // my_player.select_new_active_quest();
    //
    // my_player.solve_quest().unwrap();
    // println!("The first quest is solved\n");
    //
    // println!(
    //     "Your quest list is updated:\n{}",
    //     my_player.show_unsolved_quests()
    // );
    // println!(
    //     "Your backup is updated:\n{}",
    //     my_player.show_backpack()
    // );
}
