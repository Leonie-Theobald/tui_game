use crate::tui::grid::{GRID_HEIGHT, GRID_WIDTH};

pub const BACKGROUND_COLOR: termion::color::Black = termion::color::Black;

pub fn colored_background() -> String {
    let mut background_line = " ".repeat(GRID_WIDTH);
    background_line.push_str("\r\n");
    let background = background_line.repeat(GRID_HEIGHT + 2);
    format!(
        "{}{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::color::Bg(BACKGROUND_COLOR),
        background,
        termion::color::Bg(termion::color::Reset),
    )
}
