use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            oil_drum_interaction::{DipBranch, OilDrumInteraction},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector, background::BACKGROUND_COLOR},
};

const COORDINATES: Vector = Vector {
    horizontal: 38,
    vertical: 15,
};
const DIMENSIONS: Vector = Vector {
    horizontal: 2,
    vertical: 1,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OilDrum {
    linked_interactions: Vec<Interaction>,
}

impl OilDrum {
    pub fn initiate() -> Self {
        Self {
            linked_interactions: vec![Interaction::OilDrum(OilDrumInteraction::DipBranch(
                DipBranch,
            ))],
        }
    }
}

impl Item for OilDrum {
    fn get_name(&self) -> &'static str {
        "Oil Drum"
    }

    fn get_description(&self) -> &'static str {
        "Drum filled with oil"
    }

    fn get_detail_view(&self) -> String {
        let uncolored_oil_drum = "
                                                          
                             _____________________        
                         ___/.................._..\\___   
                        /...................../ \\....\\  
                       |\\_....................\\ /...._/|
                       |..\\_________________________/..| 
                       |...............................|  
                       |...............................|  
                       |...............................|  
                       |...............................|  
                       |...........╔═════╗.............|  
                       |...........║wOILw║.............|  
                       |...........╚═════╝.............|  
                       |...............................|  
                       |...............................|  
                       |...............................|  
                       |...............................|  
                       |.E.=.3.........................|  
                       |...............................|  
                        \\_..........................._/  
                          \\_________________________/    
                                                         ";

        let mut colored_oil_drum = String::new();
        for (vertical_offset, line) in uncolored_oil_drum.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c @ ('E' | '=' | '3') => colored_oil_drum.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(160)),
                        termion::color::Fg(termion::color::AnsiValue(204)),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                    c @ ('╔' | '═' | '╗' | '║' | '╚' | '╝' | 'O' | 'I' | 'L') => {
                        colored_oil_drum.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Bg(termion::color::White),
                            termion::color::Fg(termion::color::Black),
                            c,
                            termion::color::Bg(termion::color::Reset),
                            termion::color::Fg(termion::color::Reset),
                        ))
                    }
                    'w' => colored_oil_drum.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::White),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    '.' => colored_oil_drum.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(160)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => colored_oil_drum.push_str(&format!(
                        "{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(160)),
                        c,
                        termion::color::Bg(termion::color::Reset),
                    )),
                }
            }
        }

        colored_oil_drum
    }
}

impl Stationary for OilDrum {
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

impl Drawable for OilDrum {
    fn draw(&self) -> String {
        format!(
            "{}{}{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            termion::color::Bg(BACKGROUND_COLOR),
            "🛢️",
            termion::color::Bg(termion::color::Reset),
        )
    }
}
