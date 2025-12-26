mod item;
mod player;
mod quest;
mod tui;

use std::io::{Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use crate::{
    item::{collected_item::CollectedItems, stationed_item::StationedItems},
    player::Player,
    tui::{Orientation, game::draw_game, grid::Grid},
};

fn main() {
    // initialize
    let grid = Grid::initiate();
    let mut player = Player::initiate();
    let mut collected_items = CollectedItems::initiate();
    let mut stationed_items = StationedItems::initiate();
    draw_game(&grid, &player, &collected_items, &stationed_items);

    // start captioning single key inputs in raw mode (single presses instead of waiting for \n)
    let mut stdout = stdout().into_raw_mode().unwrap();
    for k in stdin().keys() {
        match k.as_ref().unwrap() {
            // player's movement
            Key::Char('a') => player.move_in_direction(&grid, &stationed_items, Orientation::West),
            Key::Char('w') => player.move_in_direction(&grid, &stationed_items, Orientation::North),
            Key::Char('d') => player.move_in_direction(&grid, &stationed_items, Orientation::East),
            Key::Char('s') => player.move_in_direction(&grid, &stationed_items, Orientation::South),

            // overview of collected items
            Key::Char('e') => collected_items.show_all_in_popup(),

            // interact with available stationed items
            Key::Char('f') => {
                stationed_items.select_and_solve_linked_quest(&grid, &player, &mut collected_items)
            }

            // leave game
            Key::Ctrl('c') => break,

            // ignore everything else
            _ => continue,
        };

        draw_game(&grid, &player, &collected_items, &stationed_items);
    }

    // clean up terminal and go back to normal mode
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
