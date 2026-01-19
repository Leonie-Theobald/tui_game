use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            fireplace_interaction::{
                FireplaceInteraction, LightBranch, RoastMarshmallow, ThrowBranchIntoFire,
            },
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 38,
    vertical: 10,
};
const DIMENSIONS: Vector = Vector {
    horizontal: 6,
    vertical: 4,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fireplace {
    linked_interactions: Vec<Interaction>,
}

impl Fireplace {
    pub fn initiate() -> Self {
        Self {
            linked_interactions: vec![
                Interaction::Fireplace(FireplaceInteraction::ThrowBranchIntoFire(
                    ThrowBranchIntoFire,
                )),
                Interaction::Fireplace(FireplaceInteraction::LightBranch(LightBranch)),
                Interaction::Fireplace(FireplaceInteraction::RoastMarshmallow(RoastMarshmallow)),
            ],
        }
    }
}

impl Item for Fireplace {
    fn get_name(&self) -> &'static str {
        "Cosy Fireplace"
    }

    fn get_description(&self) -> &'static str {
        "The fireplace is burning."
    }

    fn get_detail_view(&self) -> String {
        let fireplace = "

 .___________________________________________________________________________.
 |xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|
 |___________________________________________________________________________|
   |xxxxx!xxxxxxxxxxx|xxxxxxxxxxx|xxxxxxxxxxx|xxxxxxxxxxx|xxxxxxxxxxx|xxxxx|
   |_____|__,________|___________|______.____}__,________}___________|_____|
   |xxxxxxxxx|xx\\xxx\\xxx\\xxx\\xxx\\xxxx|xxx|xxxx/xxx/xxx/xxx/xxx/xx|xxxxxxxxx|
   |_________|___\\_,_\\___\\___\\.__|___|___|___|___/___/___/___/___|_________|
   |xxxxx|xxx|xxxxx|                                       |xxxxx|xxx|xxxxx|
   |_____|___|xxxxx|                                       |xxxxx|___|_____|
   |xxxxxxxxx|xxxxx|                                       |xxxxx|xxxxxxxxx|
   |_._______|xxxxx|                                       |xxxxx|_________|
   |xxxxx|xxx|xxxxx|                                       |xxxxx|xxx|xxxxx|
   |_____|__,|xxxxx|                                       |xxxxx|___|_____|
   |xxxxxxxxx|xxxxx|                                       |xxxxx|xxxxxxxxx|
   |_________|xxxx/                                         \\xxxx|_____.___|
   |xxxxx|xxx|xxx/                                           \\xxx|xxx|xxxxx|
   |___._|___|xx/                                             \\xx|___|_____|
   |xxxxxxxxx|x/                                               \\x|xxxxxxxxx|
   |_________|/                                                 \\|_________|";
        let fire = "

                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                                    
                                  .                                                
                          \\/ ,\".). ',     \\/. ',                               
                     . ' ,'|// / (/ ,.  ' ,\\(   / ,                            
                   ,   '|\\/ \\|  \\\\,' ,'.' ||  \\\\,' .'    ,   '                 
                 \" ,|' .| \\_ |\\/ |#\\_/#| /_|\\_ |#\\_/,) ',. ,'                  
                 #|(,'  )  \\\\\\#\\ \\##/  ||/ \\#\\  \\###/\\ \\/ \\/.',                
                 || ) | \\ |/ /#/ |#( \\|\\\\|  |#/ /##(  )|  \\\\'/                 
                  \\ |.\\\\ |\\_/#| /#)|##\\ \\\\_/#| |######\\ \\\\/#|                  
                  /\\\\_/# |#\\##/# \\#/     \\|/ /  @\\# #@#\\ /#/\\                  
                 /  ##/#  ##\\/#@####@@###V###\\/#@####@@#Y###                    
                  :Y##@@##@##@@###@@@###@##@@###@@@###@####:";
        let stones = "

                                                                               
                                                                               
                                                                               
                                                                               
                                                                               
                                                                                    
                                                                                   
                                                                                  
                                                                                
                                                                                     
                                                                                     
                                                                                          
                                                                                     
                                                                                         
                                                                                       
                                                                                  
                                                                                
                 :;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:                   
                   ::::::::::::::::::::::::::::::::                             ";

        let mut colored_fireplace = String::new();
        for (vertical_offset, line) in fireplace.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => {
                        let transformed_char = if c == 'x' { ' ' } else { c };
                        colored_fireplace.push_str(&format!(
                            "{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Bg(termion::color::AnsiValue(88)),
                            transformed_char,
                            termion::color::Bg(termion::color::Reset),
                        ))
                    }
                }
            }
        }

        let mut colored_fire = String::new();
        for (vertical_offset, line) in fire.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => colored_fire.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(166)),
                        termion::color::Bg(termion::color::AnsiValue(214)),
                        c,
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                }
            }
        }

        let mut colored_stones = String::new();
        for (vertical_offset, line) in stones.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => colored_stones.push_str(&format!(
                        "{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(8)),
                        c,
                        termion::color::Bg(termion::color::Reset),
                    )),
                }
            }
        }

        format!("{}{}{}", colored_fireplace, colored_stones, colored_fire)
    }
}

impl Stationary for Fireplace {
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

impl Drawable for Fireplace {
    fn draw(&self) -> String {
        let fireplace = "🧱🧱🧱
🪵🔥🧱
🪵🔥🧱
🧱🧱🧱";

        let mut fireplace_string = String::new();
        for (index, line) in fireplace.lines().enumerate() {
            fireplace_string.push_str(&format!(
                "{}{}",
                termion::cursor::Goto(
                    (COORDINATES.horizontal + 1) as u16,
                    (COORDINATES.vertical + index + 1) as u16
                ),
                line
            ))
        }

        fireplace_string
    }
}
