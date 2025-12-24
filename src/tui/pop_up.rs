use crate::tui::{
    Drawable, TuiError,
    grid::{GRID_HEIGHT, GRID_WIDTH},
};
use std::{
    fmt::Display,
    io::{Write, stdin, stdout},
};
use termion::{color, event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Debug, Default, PartialEq)]
pub enum Button {
    #[default]
    Confirm,
    Cancel,
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut button_text = String::from("<");
        button_text.push_str(match self {
            Button::Confirm => "OK",
            Button::Cancel => "Cancel",
        });
        button_text.push('>');

        write!(f, "{}", button_text)
    }
}

#[derive(Debug)]
pub struct PopUp<T: Display> {
    pub options: Vec<T>,
    pub active_option_index: usize,
    pub buttons: Vec<Button>,
    pub active_button_index: usize,
}

impl<T: Display> PopUp<T> {
    fn select_next_option(&mut self) {
        if self.active_option_index < self.options.len() - 1 {
            self.active_option_index += 1
        }
    }

    fn select_previous_option(&mut self) {
        if self.active_option_index > 0 {
            self.active_option_index -= 1
        }
    }

    fn select_next_button(&mut self) {
        if self.active_button_index < self.buttons.len() - 1 {
            self.active_button_index += 1
        }
    }

    fn select_previous_button(&mut self) {
        if self.active_button_index > 0 {
            self.active_button_index -= 1
        }
    }

    fn check_and_handle_empty(&self) -> Result<(), TuiError> {
        let mut stdout = stdout().into_raw_mode().unwrap();
        if self.options.is_empty() {
            write!(
                stdout,
                "{}{}{}",
                termion::clear::All,
                Self::show_empty_options(),
                termion::cursor::Hide
            )
            .unwrap();
            stdout.flush().unwrap();

            // wait for correct user confirmation to abort
            for k in stdin().keys() {
                if let Key::Char('\n') = k.as_ref().unwrap() {
                    return Err(TuiError::NoSelectionOptions);
                }
            }
        }

        Ok(())
    }

    fn draw_in_terminal(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        // Draw initial state
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            self.draw(),
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    pub fn get_user_selection(&mut self) -> Result<usize, TuiError> {
        self.check_and_handle_empty()?;
        self.draw_in_terminal();

        let mut stdout = stdout().into_raw_mode().unwrap();
        // react to user input
        for k in stdin().keys() {
            match k.as_ref().unwrap() {
                Key::Up => self.select_previous_option(),

                Key::Down => self.select_next_option(),

                Key::Left => self.select_previous_button(),
                Key::Right => self.select_next_button(),
                Key::Char('\n') => match self
                    .buttons
                    .get(self.active_button_index)
                    .expect("Changed index only within available range")
                {
                    // return respective user selection
                    Button::Cancel => return Err(TuiError::NoSelection),
                    Button::Confirm => return Ok(self.active_option_index),
                },
                // ignore all other key presses
                _ => continue,
            }

            // draw updated selection view
            write!(
                stdout,
                "{}{}{}",
                termion::clear::All,
                self.draw(),
                termion::cursor::Hide
            )
            .unwrap();
            stdout.flush().unwrap();
        }

        Ok(self.active_option_index)
    }

    fn show_empty_options() -> String {
        let no_options_line = format!(
            "{}{}{}{}{}{}{}",
            Self::draw_background(),
            termion::cursor::Goto(4, 3),
            termion::style::Italic,
            termion::color::Bg(termion::color::Blue),
            "No options available",
            termion::style::Reset,
            termion::color::Bg(termion::color::Reset),
        );

        let confirmation_line = format!(
            "{}{}{}{}",
            termion::cursor::Goto(4, (GRID_HEIGHT - 2) as u16),
            termion::color::Bg(termion::color::LightBlack),
            "<Cancel>",
            termion::color::Bg(termion::color::Reset),
        );

        format!("{}{}", no_options_line, confirmation_line)
    }

    fn draw_background() -> String {
        let background_line = format!(
            "{}{}{}",
            termion::color::Bg(termion::color::Blue),
            " ".repeat(GRID_WIDTH - 2),
            color::Bg(color::Reset),
        );

        let mut background = String::new();
        for index in 0..GRID_HEIGHT - 2 {
            background.push_str(&format!(
                "{}{}",
                termion::cursor::Goto(2, 2 + index as u16),
                background_line
            ))
        }

        background
    }
}

impl<T: Display> Drawable for PopUp<T> {
    fn draw(&self) -> String {
        // build options
        let mut joined_options = String::new();
        for (index, option) in self.options.iter().enumerate() {
            let line = {
                // the selected option's background is different
                if index == self.active_option_index {
                    format!(
                        "{}{}{}{}",
                        termion::cursor::Goto(4, 3 + index as u16),
                        termion::color::Bg(color::LightBlack),
                        option,
                        termion::color::Bg(color::Reset)
                    )
                } else {
                    format!(
                        "{}{}{}{}",
                        termion::cursor::Goto(4, 3 + index as u16),
                        color::Bg(color::Blue),
                        option,
                        color::Bg(color::Reset),
                    )
                }
            };
            joined_options.push_str(&line);
        }

        // build buttons
        let horizonal_spacing = if self.buttons.is_empty() {
            // there is no button and therefore no spacing is needed
            0
        } else {
            // eg three buttons:
            // |    <B1>  <B2>  <B3>    |
            // 2x margin + 3x button space = GRID_WIDTH
            // => button_space = (GRID_WIDTH - 2x margin) / 3
            (GRID_WIDTH - 8) / self.buttons.len()
        };
        let mut buttons_line = String::new();
        for (index, button) in self.buttons.iter().enumerate() {
            // change background color with active selection of button
            let button_string = if index == self.active_button_index {
                format!(
                    "{}{}{}{}",
                    termion::cursor::Goto(
                        (4 + index * horizonal_spacing) as u16,
                        (GRID_HEIGHT - 2) as u16
                    ),
                    termion::color::Bg(color::LightBlack),
                    button,
                    termion::color::Bg(termion::color::Reset),
                )
            } else {
                format!(
                    "{}{}{}{}",
                    termion::cursor::Goto(
                        (4 + index * horizonal_spacing) as u16,
                        (GRID_HEIGHT - 2) as u16
                    ),
                    termion::color::Bg(color::Blue),
                    button,
                    termion::color::Bg(termion::color::Reset),
                )
            };

            buttons_line.push_str(&button_string);
        }

        // merge background and written quests
        format!(
            "{}{}{}",
            Self::draw_background(),
            joined_options,
            buttons_line,
        )
    }
}
