mod item;
mod player;
mod quest_log;
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
    quest_log::{
        QuestLog,
        quest::{IsQuest, Quest},
    },
    tui::{Orientation, grid::Grid},
};

fn main() {
    println!("INF: Game is initiated");

    let mut my_player = Player::initiate();
    let mut my_quests = QuestLog::initiate();
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
            Key::Char('e') => my_collected_items.toggle_visibility(),
            Key::Char('r') => {
                my_quests.select_new_active_quest();
                let target_coordinates = my_player.get_facing_coordinates();
                // let accessible_stationary_items =
                //     my_stationed_items.accessible_stationary_items(target_coordinates);
                my_quests
                    .solve_quest(&mut my_collected_items)
                    .expect("Couldn't solve quest");
            }
            Key::Char('f') => {
                let target_coordinates = my_player.get_facing_coordinates();
                let mut accessible_items =
                    my_stationed_items.accessible_stationary_items(target_coordinates);
                let mut first_accessible_item = accessible_items.first_mut().unwrap();
                let linked_quest =
                    mut_method_on_inner_stationed_item!(first_accessible_item, get_linked_quest);

                let mut new_stationed_items = match linked_quest {
                    Quest::OpenRustyLock(inner) => inner.solve(&mut my_collected_items),
                    Quest::LightCandle(inner) => inner.solve(&mut my_collected_items),
                    Quest::UnwrapPacket(inner) => inner.solve(&mut my_collected_items),
                    Quest::UnlockTreasureChest(inner) => inner.solve(&mut my_collected_items),
                };
                my_stationed_items.list.append(&mut new_stationed_items);
            }
            Key::Ctrl('c') => break,
            _ => {
                write!(stdout, "\r\nUnknown character... Exiting\r\n").unwrap();
                break;
            }
        };

        write!(
            stdout,
            "{}{}{}{}{}{}{}",
            termion::clear::All,
            grid.draw(),
            my_player.draw(),
            my_quests.draw(),
            my_collected_items.draw(),
            my_stationed_items.draw(),
            termion::cursor::Hide,
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
