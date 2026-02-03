use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            treasure_chest_interaction::{TreasureChestInteraction, Unlock},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};
use serde::{Deserialize, Serialize};

const COORDINATES: Vector = Vector {
    horizontal: 31,
    vertical: 4,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TreasureChest {
    opened: bool,
    linked_interactions: Vec<Interaction>,
}

impl TreasureChest {
    pub fn initiate() -> Self {
        let linked_interactions = vec![Interaction::TreasureChest(
            TreasureChestInteraction::Unlock(Unlock),
        )];

        Self {
            opened: false,
            linked_interactions,
        }
    }
}

impl Item for TreasureChest {
    fn get_name(&self) -> &'static str {
        "Treasure Chest"
    }

    fn get_description(&self) -> &'static str {
        "Open this treasure chest with a key."
    }

    fn get_detail_view(&self) -> String {
        let uncolored_chest = if self.opened {
            "
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                      _______________________________                                                          
                      \\..............................\\                           
                       \\______________________________\\
                       /|............................/|
                      /-┵---------------------------/.|
                      |..┏━━━━━━━━━━━━━━━━━━━━━━━┓..|.|
                      |..┃aaaaaaaa╔═════╗aaaaaaaa┃..|.|
                      |..┃aaaaaaaa║bbobb║aaaaaaaa┃..|.|
                      |..┃aaaaaaaa╚═════╝aaaaaaaa┃..|.|
                      |..┃aaaaaaaaaaaaaaaaaaaaaaa┃..|.|
                      |..┃aaaaaaaaaaaaaaaaaaaaaaa┃..|.|
                      |..┗━━━━━━━━━━━━━━━━━━━━━━━┛..|.|
                      |_____________________________|/"
        } else {
            "
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                        ______________________________
                       /............................./|
                      /-----------------------------/.|
                      |..┏━━━━━━━━━━━━━━━━━━━━━━━┓..|.|
                      |..┃aaaaaaaa╔═════╗aaaaaaaa┃..|.|
                      |..┃aaaaaaaa║bbobb║aaaaaaaa┃..|.|
                      |..┃aaaaaaaa╚═════╝aaaaaaaa┃..|.|
                      |..┃aaaaaaaaaaaaaaaaaaaaaaa┃..|.|
                      |..┃aaaaaaaaaaaaaaaaaaaaaaa┃..|.|
                      |..┗━━━━━━━━━━━━━━━━━━━━━━━┛..|.|
                      |_____________________________|/"
        };

        let mut colored_chest = String::new();
        for (vertical_offset, line) in uncolored_chest.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c @ ('┵' | '\\' | '/' | '_' | '-' | '|') => colored_chest.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(237)),
                        termion::color::Bg(termion::color::AnsiValue(130)),
                        c,
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    '.' => colored_chest.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(130)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c @ ('┏' | '┃' | '━' | '┓' | '┗' | '┛') => {
                        colored_chest.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Fg(termion::color::AnsiValue(94)),
                            termion::color::Bg(termion::color::AnsiValue(215)),
                            c,
                            termion::color::Fg(termion::color::Reset),
                            termion::color::Bg(termion::color::Reset),
                        ))
                    }
                    'a' => colored_chest.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(215)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c @ ('╔' | '═' | '╗' | '║' | '╚' | '╝') => {
                        colored_chest.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Fg(termion::color::AnsiValue(236)),
                            termion::color::Bg(termion::color::AnsiValue(240)),
                            c,
                            termion::color::Fg(termion::color::Reset),
                            termion::color::Bg(termion::color::Reset),
                        ))
                    }
                    'b' => colored_chest.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(240)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    'o' => colored_chest.push_str(&format!(
                        "{}{}{}o{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(236)),
                        termion::color::Bg(termion::color::AnsiValue(240)),
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => {
                        panic!("The character '{c}' should not be part of treasure chest drawing.")
                    }
                }
            }
        }

        colored_chest
    }
}

impl Stationary for TreasureChest {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinate = Vec::new();

        // +-1 around the symbol is also blocked
        let horizontal_range = COORDINATES.horizontal..COORDINATES.horizontal + 2;
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
        if self.opened {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.opened = true;
    }
}

impl Drawable for TreasureChest {
    fn draw(&self) -> String {
        let symbol = if self.opened { "X" } else { "🧳" };

        format!(
            "{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            symbol,
        )
    }
}
