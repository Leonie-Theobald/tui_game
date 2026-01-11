use std::io::{Write, stdout};
use termion::raw::IntoRawMode;

use crate::{
    item::{collected_item::CollectedItems, stationed_item::StationedItems},
    player::Player,
    quest::Quest,
    tui::{Drawable, grid::Grid, status_bar},
};

pub fn draw_game(
    grid: &Grid,
    player: &Player,
    collected_items: &CollectedItems,
    stationed_items: &StationedItems,
    quests: &[Quest],
) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}{}{}{}",
        termion::clear::All,
        grid.draw(),
        player.draw(),
        stationed_items.draw(),
        status_bar::draw_status_bar(stationed_items, collected_items, player, quests),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}
