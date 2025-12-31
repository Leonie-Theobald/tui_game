use crate::tui::TuiError;
use inquire::{InquireError, Select};
use std::{
    fmt::Display,
    io::{Write, stdout},
};
use termion::raw::IntoRawMode;

#[derive(Debug)]
pub struct PopUp<T: Display> {
    pub options: Vec<T>,
}

impl<T: Display> PopUp<T> {
    fn check_and_handle_empty(&self) -> Result<(), TuiError> {
        let mut stdout = stdout().into_raw_mode().unwrap();

        if self.options.is_empty() {
            // prepare terminal
            write!(
                stdout,
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide
            )
            .unwrap();
            stdout.flush().unwrap();

            // Show selection prompt with no real options
            match Select::new("User Selection", vec!["No options available"]).prompt() {
                // It doesn't matter if the user confirms the "no options" or cancels via ESC or
                // CTRL<C>
                Ok(_)
                | Err(InquireError::OperationCanceled)
                | Err(InquireError::OperationInterrupted) => {
                    return Err(TuiError::NoSelectionOptions);
                }
                Err(err) => panic!("Error shouldn't occur: {:?}", err),
            }
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
