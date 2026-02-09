use crate::tui::{
    TuiError,
    background::{BACKGROUND_COLOR, colored_background},
};
use inquire::{
    InquireError, Select,
    ui::{RenderConfig, StyleSheet, Styled},
};
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

        let general_infos = format!(
            "{}{}{}{}Press BACKSPACE to go back{}{}{}",
            termion::color::Bg(BACKGROUND_COLOR),
            termion::style::Italic,
            termion::color::Fg(color::LightBlack),
            termion::cursor::Goto(1, 4),
            termion::style::Reset,
            termion::color::Fg(color::Reset),
            termion::color::Bg(color::Reset),
        );

        let specific_message = format!(
            "{}{}{}{}",
            termion::cursor::Goto(1, 2),
            termion::color::Bg(BACKGROUND_COLOR),
            self.message,
            termion::color::Bg(color::Reset),
        );

        // Write info message
        write!(
            stdout,
            "{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Hide,
            colored_background(),
            general_infos,
            specific_message
        )
        .unwrap();
        stdout.flush().unwrap();

        for k in stdin().keys() {
            match k.as_ref().unwrap() {
                event::Key::Backspace => return,
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
            "{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            colored_background(),
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .unwrap();
        stdout.flush().unwrap();

        // define inquire rendering
        let render_config = RenderConfig {
            prompt_prefix: Styled {
                content: "?",
                style: StyleSheet::new().with_bg(inquire::ui::Color::Black),
            },
            prompt: StyleSheet::new().with_bg(inquire::ui::Color::Black),
            help_message: StyleSheet::new()
                .with_bg(inquire::ui::Color::Black)
                .with_fg(inquire::ui::Color::White),
            answer: StyleSheet::new().with_bg(inquire::ui::Color::Black),
            highlighted_option_prefix: Styled {
                content: ">",
                style: StyleSheet::new().with_bg(inquire::ui::Color::Black),
            },
            option: StyleSheet::new().with_bg(inquire::ui::Color::Black),
            selected_option: Some(StyleSheet::new().with_bg(inquire::ui::Color::Black)),
            ..Default::default()
        };

        // Show selection options
        match Select::new("User Selection", self.options)
            .with_render_config(render_config)
            .prompt()
        {
            Ok(selection) => Ok(selection),
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                Err(TuiError::NoSelection)
            }
            Err(err) => panic!("Error shouldn't occur: {:?}", err),
        }
    }
}
