use crate::tui::TuiError;
use inquire::{InquireError, Select};
use std::{
    fmt::Display,
    io::{Write, stdin, stdout},
};
use termion::{color, event, input::TermRead, raw::IntoRawMode};

pub struct InfoPopUp {
    pub message: String,
}

impl InfoPopUp {
    pub fn show_to_user(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        // clear terminal
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();

        // Write general infos
        write!(
            stdout,
            "{}{}{}Press ENTER to continue{}{}",
            termion::style::Italic,
            termion::color::Fg(color::LightBlack),
            termion::cursor::Goto(1, 4),
            termion::style::Reset,
            termion::color::Fg(color::Reset),
        )
        .unwrap();

        // Write info message
        write!(stdout, "{}{}", termion::cursor::Goto(1, 2), self.message).unwrap();
        stdout.flush().unwrap();

        for k in stdin().keys() {
            match k.as_ref().unwrap() {
                event::Key::Char('\n') => return,
                // ignore everything else
                _ => continue,
            };
        }
    }
}

#[derive(Debug)]
pub struct PopUp<T: Display> {
    pub options: Vec<T>,
}

impl<T: Display> PopUp<T> {
    fn check_and_handle_empty(&self) -> Result<(), TuiError> {
        if self.options.is_empty() {
            InfoPopUp {
                message: String::from("No options available."),
            }
            .show_to_user();
            return Err(TuiError::NoSelectionOptions);
        }

        Ok(())
    }

    pub fn get_user_selection(self) -> Result<T, TuiError> {
        self.check_and_handle_empty()?;

        // prepare terminal
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        // Show selection options
        match Select::new("User Selection", self.options).prompt() {
            Ok(selection) => Ok(selection),
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                Err(TuiError::NoSelection)
            }
            Err(err) => panic!("Error shouldn't occur: {:?}", err),
        }
    }
}
