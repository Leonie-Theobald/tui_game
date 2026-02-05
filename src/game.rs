use std::io::{Write, stdin, stdout};
use termion::{event, input::TermRead, raw::IntoRawMode};

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

pub fn explain_game() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut info_prompt = String::new();
    info_prompt.push_str(&format!(
        "{}{}{}Game Explanation{}",
        termion::cursor::Goto(2, 1),
        termion::style::Bold,
        termion::style::Italic,
        termion::style::Reset
    ));
    info_prompt.push_str(&format!(
        "{}You are this: {}▲{}",
        termion::cursor::Goto(2, 2),
        termion::color::Fg(termion::color::Magenta),
        termion::color::Fg(termion::color::Reset),
    ));
    info_prompt.push_str(&format!(
        "{}You control your avatar by pressing w a s d",
        termion::cursor::Goto(2, 3),
    ));
    info_prompt.push_str(&format!(
        "{}You can interact with objects by pressing e",
        termion::cursor::Goto(2, 4),
    ));
    info_prompt.push_str(&format!(
        "{}You can view your collected items by pressing c",
        termion::cursor::Goto(2, 5),
    ));
    info_prompt.push_str(&format!(
        "{}You can view your current quests by pressing q",
        termion::cursor::Goto(2, 6),
    ));
    info_prompt.push_str(&format!(
        "{}You can leave the game by pressing ESC",
        termion::cursor::Goto(2, 7),
    ));
    info_prompt.push_str(&format!(
        "{}You can view these explanations again by pressing ?",
        termion::cursor::Goto(2, 8),
    ));
    info_prompt.push_str(&format!(
        "{}Walk around, take a look, have fun!",
        termion::cursor::Goto(2, 10),
    ));

    info_prompt.push_str(&format!(
        "{}{}{}Press ENTER to continue{}{}",
        termion::cursor::Goto(2, 12),
        termion::color::Fg(termion::color::LightBlack),
        termion::style::Italic,
        termion::color::Fg(termion::color::Reset),
        termion::style::Reset,
    ));

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        info_prompt,
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    for k in stdin().keys() {
        match k.as_ref().unwrap() {
            event::Key::Char('\n') => return,
            // ignore everything else
            _ => continue,
        };
    }
}

pub fn start_game() -> (Player, CollectedItems, StationedItems, Quests) {
    let loading_game_prompt = PopUp {
        options: vec!["Load existing game", "Start new game"],
    };
    match loading_game_prompt.get_user_selection() {
        Ok("Load existing game") => QuickSave::load_from_file().disassemble_to_game_state(),
        // for new game or aborted selection
        _ => {
            explain_game();

            (
                Player::initiate(),
                CollectedItems::initiate(),
                StationedItems::initiate(),
                Quests::initiate(),
            )
        }
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
