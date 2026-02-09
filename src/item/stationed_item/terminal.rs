use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            terminal_interaction::{EnterDigits, TerminalInteraction, password::Password},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector, background::BACKGROUND_COLOR},
};

const COORDINATES: Vector = Vector {
    horizontal: 30,
    vertical: 7,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Terminal {
    pub correct_code_entered: bool,
    linked_interactions: Vec<Interaction>,
}

impl Terminal {
    pub fn initiate() -> Self {
        let linked_interactions = vec![Interaction::Terminal(TerminalInteraction::EnterDigits(
            EnterDigits,
        ))];

        Self {
            correct_code_entered: false,
            linked_interactions,
        }
    }
}

impl Item for Terminal {
    fn get_name(&self) -> &'static str {
        "Terminal"
    }

    fn get_description(&self) -> &'static str {
        "Small display and number pad"
    }

    fn get_detail_view(&self) -> String {
        let horizontal_origin = 25;
        let vertical_origin = 3;

        let terminal_frame = "┌─────────────────────────┐
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
│.........................│
└─────────────────────────┘";
        let mut colored_terminal_frame = String::new();
        for (vertical_offset, line) in terminal_frame.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    '.' => colored_terminal_frame.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_origin + horizontal_offset) as u16,
                            (vertical_origin + vertical_offset) as u16
                        ),
                        termion::color::Bg(termion::color::LightBlack),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => colored_terminal_frame.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_origin + horizontal_offset) as u16,
                            (vertical_origin + vertical_offset) as u16
                        ),
                        termion::color::Bg(termion::color::LightBlack),
                        termion::color::Fg(termion::color::LightBlack),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }

        let display = "┌─────────────────────┐
│.....................│
│.....................│
│.....................│
│.....................│
│.....................│
└─────────────────────┘";
        let mut colored_display = String::new();
        for (vertical_offset, line) in display.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    '.' => colored_display.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_origin + horizontal_offset + 2) as u16,
                            (vertical_origin + vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::LightBlack),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => colored_display.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_origin + horizontal_offset + 2) as u16,
                            (vertical_origin + vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::LightBlack),
                        termion::color::Fg(termion::color::AnsiValue(46)),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }

        let number_pad = "┌───┰───┰───┐
│.7.│.8.│.9.│
┝───┼───┼───┨
│.4.│.5.│.6.│
┝───┼───┼───┨
│.1.│.2.│.3.│
┝───┼───┼───┨
│...│.0.│...│
└───┴───┴───┘";
        let mut colored_number_pad = String::new();
        for (vertical_offset, line) in number_pad.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    '.' => colored_number_pad.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_origin + horizontal_offset + 7) as u16,
                            (vertical_origin + vertical_offset + 8) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(222)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => colored_number_pad.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_origin + horizontal_offset + 7) as u16,
                            (vertical_origin + vertical_offset + 8) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(222)),
                        termion::color::Fg(termion::color::Black),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }

        let prompt_message = {
            if self.correct_code_entered {
                format!(
                    "{}{}{}Correct Password:{}{}",
                    termion::cursor::Goto(
                        (horizontal_origin + 6) as u16,
                        (vertical_origin + 3) as u16
                    ),
                    termion::color::Bg(termion::color::LightBlack),
                    termion::color::Fg(termion::color::AnsiValue(46)),
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )
            } else {
                format!(
                    "{}{}{}Enter Password:{}{}",
                    termion::cursor::Goto(
                        (horizontal_origin + 6) as u16,
                        (vertical_origin + 3) as u16
                    ),
                    termion::color::Bg(termion::color::LightBlack),
                    termion::color::Fg(termion::color::AnsiValue(46)),
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )
            }
        };

        let password_line = {
            if self.correct_code_entered {
                format!(
                    "{}{}{}{}{}{}",
                    termion::cursor::Goto(
                        (horizontal_origin + 6) as u16,
                        (vertical_origin + 4) as u16
                    ),
                    termion::color::Bg(termion::color::LightBlack),
                    termion::color::Fg(termion::color::AnsiValue(46)),
                    Password::get_correct(),
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )
            } else {
                format!(
                    "{}{}{}******{}{}",
                    termion::cursor::Goto(
                        (horizontal_origin + 6) as u16,
                        (vertical_origin + 4) as u16
                    ),
                    termion::color::Bg(termion::color::LightBlack),
                    termion::color::Fg(termion::color::AnsiValue(46)),
                    termion::color::Bg(termion::color::Reset),
                    termion::color::Fg(termion::color::Reset),
                )
            }
        };

        format!(
            "{}{}{}{}{}",
            colored_terminal_frame,
            colored_display,
            colored_number_pad,
            prompt_message,
            password_line
        )

        // ┌─────────────────────────┐
        // │ ┌─────────────────────┐ │
        // │ │                     │ │
        // │ │   Enter Password:   │ │
        // │ │   ******            │ │
        // │ │                     │ │
        // │ │                     │ │
        // │ └─────────────────────┘ │
        // │      ┌───┰───┰───┐      │
        // │      │ 7 │ 8 │ 9 │      │
        // │      ┝───┼───┼───┨      │
        // │      │ 4 │ 5 │ 6 │      │
        // │      ┝───┼───┼───┨      │
        // │      │ 1 │ 2 │ 3 │      │
        // │      ┝───┼───┼───┨      │
        // │      │   │ 0 │   │      │
        // │      └───┴───┴───┘      │
        // └─────────────────────────┘
        //
        //
        //
        //
        //
        //
        //         } else {
        // "┌─────────────────────────┐
        // │ ┌─────────────────────┐ │
        // │ │                     │ │
        // │ │        Enter        │ │
        // │ │      Password       │ │
        // │ │       ******        │ │
        // │ │                     │ │
        // │ └─────────────────────┘ │
        // │      ┌───┰───┰───┐      │
        // │      │ 7 │ 8 │ 9 │      │
        // │      ┝───┼───┼───┨      │
        // │      │ 4 │ 5 │ 6 │      │
        // │      ┝───┼───┼───┨      │
        // │      │ 1 │ 2 │ 3 │      │
        // │      ┝───┼───┼───┨      │
        // │      │   │ 0 │   │      │
        // │      └───┴───┴───┘      │
        // └─────────────────────────┘
        //         }
    }
}

impl Stationary for Terminal {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let width = if self.correct_code_entered { 2 } else { 5 };

        let mut blocked_coordinate = Vec::new();

        // +-1 around the symbol is also blocked
        let horizontal_range = COORDINATES.horizontal..COORDINATES.horizontal + width;
        for horizontal_index in horizontal_range {
            let vertical_range = COORDINATES.vertical..COORDINATES.vertical + 1;
            for vertical_index in vertical_range {
                blocked_coordinate.push(Vector {
                    horizontal: horizontal_index,
                    vertical: vertical_index,
                })
            }
        }

        blocked_coordinate
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        if self.correct_code_entered {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.correct_code_entered = true;
    }
}

impl Drawable for Terminal {
    fn draw(&self) -> String {
        if self.correct_code_entered {
            format!(
                "{}{}📟{}",
                termion::cursor::Goto(
                    (COORDINATES.horizontal + 1) as u16,
                    (COORDINATES.vertical + 1) as u16
                ),
                termion::color::Bg(BACKGROUND_COLOR),
                termion::color::Bg(termion::color::Reset),
            )
        } else {
            format!(
                "{}{}📟{}███{}{}",
                termion::cursor::Goto(
                    (COORDINATES.horizontal + 1) as u16,
                    (COORDINATES.vertical + 1) as u16
                ),
                termion::color::Bg(BACKGROUND_COLOR),
                termion::color::Fg(termion::color::AnsiValue(136)),
                termion::color::Bg(termion::color::Reset),
                termion::color::Fg(termion::color::Reset),
            )
        }
    }
}
