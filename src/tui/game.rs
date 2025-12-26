use std::io::{Write, stdout};
use termion::raw::IntoRawMode;

use crate::{
    item::{collected_item::CollectedItems, stationed_item::StationedItems},
    player::Player,
    tui::{Drawable, grid::Grid},
};

pub fn draw_game(
    grid: &Grid,
    player: &Player,
    collected_items: &CollectedItems,
    stationed_items: &StationedItems,
) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}{}{}{}",
        termion::clear::All,
        grid.draw(),
        player.draw(),
        collected_items.draw(),
        stationed_items.draw(),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}
