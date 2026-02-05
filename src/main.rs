mod final_cut_scene;
mod game;
mod inspection;
mod item;
mod player;
mod quest;
mod quick_save;
mod tui;

use std::io::{stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use crate::{
    game::{draw_game, save_and_leave_game, start_game},
    tui::{Orientation, clean_up::clean_up_terminal, grid::Grid},
};

fn main() {
    // restore or initialize new game
    let (mut player, mut collected_items, mut stationed_items, mut quests) = start_game();
    let grid = Grid::initiate();
    draw_game(&grid, &player, &collected_items, &stationed_items, &quests);

    // start captioning single key inputs in raw mode (single presses instead of waiting for \n)
    let _stdout = stdout().into_raw_mode().unwrap();

    for k in stdin().keys() {
        match k.as_ref().unwrap() {
            // player's movement
            Key::Char('a') => player.move_in_direction(&grid, &stationed_items, Orientation::West),
            Key::Char('w') => player.move_in_direction(&grid, &stationed_items, Orientation::North),
            Key::Char('d') => player.move_in_direction(&grid, &stationed_items, Orientation::East),
            Key::Char('s') => player.move_in_direction(&grid, &stationed_items, Orientation::South),

            // overview of collected items
            Key::Char('c') => collected_items.show_all_in_popup(),

            // interact with available stationed items
            Key::Char('e') => {
                stationed_items.interact(&player, &mut collected_items);
            }

            // show quests
            Key::Char('q') => quests.show_open_quests(),

            // leave game
            Key::Esc => {
                save_and_leave_game(player, collected_items, stationed_items, quests);
                break;
            }

            // ignore everything else
            _ => continue,
        };

        quests.check_quests(&mut collected_items, &mut stationed_items);
        draw_game(&grid, &player, &collected_items, &stationed_items, &quests);
    }

    clean_up_terminal();
}
