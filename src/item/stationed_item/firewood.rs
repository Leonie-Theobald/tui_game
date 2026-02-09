use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            firewood_interaction::{ChopSkewer, FirewoodInteraction, TakeBranch},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector, background::BACKGROUND_COLOR},
};

const COORDINATES: Vector = Vector {
    horizontal: 38,
    vertical: 8,
};
const DIMENSIONS: Vector = Vector {
    horizontal: 2,
    vertical: 1,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Firewood {
    linked_interactions: Vec<Interaction>,
}

impl Firewood {
    pub fn initiate() -> Self {
        Self {
            linked_interactions: vec![
                Interaction::Firewood(FirewoodInteraction::TakeBranch(TakeBranch)),
                Interaction::Firewood(FirewoodInteraction::ChopSkewer(ChopSkewer)),
            ],
        }
    }
}

impl Item for Firewood {
    fn get_name(&self) -> &'static str {
        "Firewood"
    }

    fn get_description(&self) -> &'static str {
        "Pile of Wood"
    }

    fn get_detail_view(&self) -> String {
        let uncolored_firewood =
            "                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                     ____ __                                    
                                   /xx_xx\\xx\\_                                  
                                   |xxO\\x|xxxx\\_                                
                           ________\\_____/______\\________                       
                          /xAx=x1xxxxxxxxxxxxxxxxx/xxxxxx\\                      
                         /xxxxxxxxxxxxxxxxxxxxxxx/xxx__xxx\\                     
                         |xxxxxxxxxxxxxxxxxxxxxxx|xx/xx\\xx|                     
                         |xxxxxxxxxxxxxxxxxxxxxxx|xx\\__/xx|                     
                         \\xxxxxxxxxxxxxxxxxxxxxxx\\xxxxxxxx/                     
                          \\_______________________\\______/";

        let mut colored_firewood = String::new();
        for (vertical_offset, line) in uncolored_firewood.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c @ ('A' | '=' | '1') => colored_firewood.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(130)),
                        termion::color::Fg(termion::color::AnsiValue(137)),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                    c => {
                        let transformed_char = if c == 'x' { ' ' } else { c };
                        colored_firewood.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Bg(termion::color::AnsiValue(130)),
                            termion::color::Fg(termion::color::Black),
                            transformed_char,
                            termion::color::Bg(termion::color::Reset),
                            termion::color::Fg(termion::color::Reset),
                        ))
                    }
                }
            }
        }

        colored_firewood
    }
}

impl Stationary for Firewood {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinates = Vec::new();
        for horizontal_offset in 0..DIMENSIONS.horizontal {
            for vertical_offset in 0..DIMENSIONS.vertical {
                blocked_coordinates.push(Vector {
                    horizontal: COORDINATES.horizontal + horizontal_offset,
                    vertical: COORDINATES.vertical + vertical_offset,
                });
            }
        }

        blocked_coordinates
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        InteractionStatus::Open
    }

    fn set_interaction_status_to_finished(&mut self) {}
}

impl Drawable for Firewood {
    fn draw(&self) -> String {
        format!(
            "{}{}{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            termion::color::Bg(BACKGROUND_COLOR),
            "🪵",
            termion::color::Bg(termion::color::Reset),
        )
    }
}
