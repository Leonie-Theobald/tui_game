use std::{
    io::{Write, stdout},
    time::Duration,
};
use termion::raw::IntoRawMode;

use crate::final_cut_scene::{room_with_cake::room_with_cake, sliding_wall_frames::WALL_FRAMES};

pub fn show_plain_wall() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let uncolored_wall =
        "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd
dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";

    let mut colored_wall = String::new();
    for (vertical_offset, line) in uncolored_wall.lines().enumerate() {
        for (horizontal_offset, _char) in line.chars().enumerate() {
            colored_wall.push_str(&format!(
                "{}{} {}",
                termion::cursor::Goto((horizontal_offset + 1) as u16, (vertical_offset + 1) as u16),
                termion::color::Bg(termion::color::AnsiValue(180)),
                termion::color::Bg(termion::color::Reset),
            ));
        }
    }

    // draw the frame on screen
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        colored_wall,
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}

pub fn slide_walls() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    for frame in WALL_FRAMES {
        let mut colored_frame = String::new();
        for (vertical_offset, line) in frame.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    'd' => colored_frame.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(180)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => colored_frame.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(180)),
                        termion::color::Fg(termion::color::Black),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }

        // draw the frame on screen
        write!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            room_with_cake(),
            colored_frame,
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        std::thread::sleep(Duration::from_millis(100));
    }
}
