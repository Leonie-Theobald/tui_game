mod item;
mod player;
mod quest;
mod tui;

use std::io::{Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use tui::Drawable;

use crate::{
    player::Player,
    tui::{Orientation, grid::Grid},
};

fn main() {
    println!("INF: Game is initiated");

    let mut my_player = Player::initiate();
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

    let mut stdout = stdout().into_raw_mode().unwrap();
    for k in stdin().keys() {
        match k.as_ref().unwrap() {
            Key::Char('a') => my_player.move_in_direction(&grid, Orientation::West),
            Key::Char('w') => my_player.move_in_direction(&grid, Orientation::North),
            Key::Char('d') => my_player.move_in_direction(&grid, Orientation::East),
            Key::Char('s') => my_player.move_in_direction(&grid, Orientation::South),
            Key::Char('q') => my_player.quests.toggle_visibility(),
            Key::Ctrl('c') => break,
            _ => {
                write!(stdout, "\r\nUnknown character... Exiting\r\n").unwrap();
                break;
            }
        };

        write!(
            stdout,
            "{}{}{}{}{}",
            termion::clear::All,
            grid.draw(),
            my_player.draw(),
            my_player.quests.draw(),
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
        "{}, you can use these items:\n{}",
        my_player.show_name(),
        my_player.show_backpack()
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
