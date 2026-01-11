mod inspection;
mod item;
mod player;
mod quest;
mod quick_save;
mod tui;

use std::io::{Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use crate::{
    item::{Item, collected_item::CollectedItems, stationed_item::StationedItems},
    player::Player,
    quest::Quest,
    quick_save::QuickSave,
    tui::{
        Orientation,
        game::draw_game,
        grid::Grid,
        pop_up::{InfoPopUp, PopUp},
    },
};

fn main() {
    // restore or initialize new game
    let loading_game_prompt = PopUp {
        options: vec!["Load existing game", "Start new game"],
    };
    let (mut player, mut collected_items, mut stationed_items, mut quests) = {
        match loading_game_prompt.get_user_selection() {
            Ok("Load existing game") => QuickSave::load_from_file().disassemble_to_game_state(),
            // for new game or aborted selection
            _ => (
                Player::initiate(),
                CollectedItems::initiate(),
                StationedItems::initiate(),
                Quest::initiate(),
            ),
        }
    };
    let grid = Grid::initiate();
    draw_game(&grid, &player, &collected_items, &stationed_items, &quests);

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
            Key::Char('c') => collected_items.show_all_in_popup(),

            // interact with available stationed items
            Key::Char('i') => {
                stationed_items.interact(&player, &mut collected_items);
            }

            // show quests
            Key::Char('q') => {
                let chained_quest_descriptions = quests
                    .iter()
                    .filter(|quest| !quest.get_inner().is_finished())
                    .fold(String::new(), |mut chained_quest_descriptions, quest| {
                        chained_quest_descriptions.push_str(&quest.get_inner().get_description());
                        chained_quest_descriptions.push_str("\r\n");
                        chained_quest_descriptions
                    });
                let info_prompt = InfoPopUp {
                    message: chained_quest_descriptions,
                };
                info_prompt.show_to_user();
            }

            // test inspection view of packet
            Key::Char('l') => {
                let inspection = item::collected_item::rusty_key::RustyKey.get_inspection();

                inspection.start_inspection();
            }

            // leave game
            Key::Esc => {
                QuickSave::from_current_game_state(
                    player,
                    collected_items,
                    stationed_items,
                    quests,
                )
                .save_to_file();
                break;
            }

            // ignore everything else
            _ => continue,
        };

        Quest::check_quests(&mut quests, &mut collected_items, &mut stationed_items);
        draw_game(&grid, &player, &collected_items, &stationed_items, &quests);
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
