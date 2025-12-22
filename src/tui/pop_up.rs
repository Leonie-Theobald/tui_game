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
pub enum ConfirmationButton {
    Cancel,
    #[default]
    Confirm,
}

#[derive(Debug, Default)]
pub struct PopUp<T: Display> {
    pub options: Vec<T>,
    pub active_option_index: usize,
    pub selected_confirmation_button: ConfirmationButton,
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

    pub fn get_user_selection(&mut self) -> Result<usize, TuiError> {
        let mut stdout = stdout().into_raw_mode().unwrap();

        if self.options.is_empty() {
            write!(
                stdout,
                "{}{}{}",
                termion::clear::All,
                self.show_empty_options(),
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

        // react to user input
        for k in stdin().keys() {
            match k.as_ref().unwrap() {
                Key::Up => {
                    self.select_previous_option();
                }
                Key::Down => {
                    self.select_next_option();
                }
                Key::Left => self.selected_confirmation_button = ConfirmationButton::Cancel,
                Key::Right => self.selected_confirmation_button = ConfirmationButton::Confirm,
                Key::Char('\n') => match self.selected_confirmation_button {
                    // return respective user selection
                    ConfirmationButton::Cancel => return Err(TuiError::NoSelection),
                    ConfirmationButton::Confirm => return Ok(self.active_option_index),
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

    fn show_empty_options(&self) -> String {
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
            color::Bg(color::Blue),
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
                    format!("{}{}", termion::cursor::Goto(4, 3 + index as u16), option)
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

        // build confirmation buttons
        // change background color with active selection of button
        let cancel_button = {
            if self.selected_confirmation_button == ConfirmationButton::Cancel {
                format!(
                    "{}{}{}{}",
                    termion::cursor::Goto(4, (GRID_HEIGHT - 2) as u16),
                    termion::color::Bg(color::LightBlack),
                    "<Cancel>",
                    termion::color::Bg(termion::color::Reset),
                )
            } else {
                format!(
                    "{}{}{}{}",
                    termion::cursor::Goto(4, (GRID_HEIGHT - 2) as u16),
                    termion::color::Bg(color::Blue),
                    "<Cancel>",
                    termion::color::Bg(termion::color::Reset),
                )
            }
        };
        let confirm_button = {
            if self.selected_confirmation_button == ConfirmationButton::Confirm {
                format!(
                    "{}{}{}{}",
                    termion::cursor::Goto((GRID_WIDTH - 8) as u16, (GRID_HEIGHT - 2) as u16),
                    termion::color::Bg(color::LightBlack),
                    "<Ok>",
                    termion::color::Bg(termion::color::Reset),
                )
            } else {
                format!(
                    "{}{}{}{}",
                    termion::cursor::Goto((GRID_WIDTH - 8) as u16, (GRID_HEIGHT - 2) as u16),
                    termion::color::Bg(color::Blue),
                    "<Ok>",
                    termion::color::Bg(termion::color::Reset),
                )
            }
        };

        // merge background and written quests
        format!(
            "{}{}{}{}",
            Self::draw_background(),
            joined_options,
            cancel_button,
            confirm_button
        )
    }
}
