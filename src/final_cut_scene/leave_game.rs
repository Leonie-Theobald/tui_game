use std::io::{Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use crate::quick_save::QuickSave;

pub fn prepare_leaving_game() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}Press L to leave{}{}",
        termion::cursor::Goto(64, 22),
        termion::color::Bg(termion::color::AnsiValue(130)),
        termion::color::Fg(termion::color::AnsiValue(88)),
        termion::color::Bg(termion::color::Reset),
        termion::color::Fg(termion::color::Reset)
    )
    .unwrap();
    stdout.flush().unwrap();

    for k in stdin().keys() {
        match k.as_ref().unwrap() {
            Key::Char('L') | Key::Char('l') => {
                QuickSave::delete_save_file();
                break;
            }
            _ => continue,
        };
    }
}
