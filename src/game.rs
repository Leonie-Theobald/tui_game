use std::io::{Write, stdout};
use termion::raw::IntoRawMode;

use crate::{
    item::{collected_item::CollectedItems, stationed_item::StationedItems},
    player::Player,
    quest::Quests,
    quick_save::QuickSave,
    tui::{Drawable, grid::Grid, pop_up::PopUp, status_bar},
};

pub fn draw_game(
    grid: &Grid,
    player: &Player,
    collected_items: &CollectedItems,
    stationed_items: &StationedItems,
    quests: &Quests,
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

pub fn start_game() -> (Player, CollectedItems, StationedItems, Quests) {
    let loading_game_prompt = PopUp {
        options: vec!["Load existing game", "Start new game"],
    };
    match loading_game_prompt.get_user_selection() {
        Ok("Load existing game") => QuickSave::load_from_file().disassemble_to_game_state(),
        // for new game or aborted selection
        _ => (
            Player::initiate(),
            CollectedItems::initiate(),
            StationedItems::initiate(),
            Quests::initiate(),
        ),
    }
}

pub fn save_and_leave_game(
    player: Player,
    collected_items: CollectedItems,
    stationed_items: StationedItems,
    quests: Quests,
) {
    QuickSave::from_current_game_state(player, collected_items, stationed_items, quests)
        .save_to_file();
}
