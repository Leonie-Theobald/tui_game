use crate::{
    item::{Item, collected_item::Collectable},
    tui::background::BACKGROUND_COLOR,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Candle;

impl Item for Candle {
    fn get_name(&self) -> &'static str {
        "Candle"
    }

    fn get_description(&self) -> &'static str {
        "A candle that is burning."
    }

    fn get_detail_view(&self) -> String {
        let flame = " 
                            
                               ,
                              (x\\
                               )x\\
                              ((\\x)
                              `-;";

        let wax = "  
                            
                                
                                  
                                   
                                    
                                    
                             |`-.---)
                             |xxx\\x(
                             |xxxx\\x)
                             |xxxx|/
                             |xxxx`|
                             |xxxxx|
                             |xxxxx|
                             |xxxxx|
                             |xxxxx|
                             |_____|
                                      
                                       
                                       ";

        let base = "  
                            
                                
                                  
                                   
                                    
                                    
                                     
                                     
                                      
                                    
                                    
                                    
                                    
                                    
                                    
                                    
                           _(_______)_
                          |xxxxxxxxxxx|
                          |___________|";

        let mut colored_flame = String::new();
        for (vertical_offset, line) in flame.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => {
                        let transformed_char = if c == 'x' { ' ' } else { c };
                        colored_flame.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Bg(termion::color::AnsiValue(214)),
                            termion::color::Fg(termion::color::AnsiValue(166)),
                            transformed_char,
                            termion::color::Bg(termion::color::Reset),
                            termion::color::Fg(termion::color::Reset),
                        ))
                    }
                }
            }
        }

        let mut colored_wax = String::new();
        for (vertical_offset, line) in wax.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => {
                        let transformed_char = if c == 'x' { ' ' } else { c };
                        colored_wax.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Bg(termion::color::AnsiValue(229)),
                            termion::color::Fg(termion::color::AnsiValue(178)),
                            transformed_char,
                            termion::color::Bg(termion::color::Reset),
                            termion::color::Fg(termion::color::Reset),
                        ))
                    }
                }
            }
        }

        let mut colored_base = String::new();
        for (vertical_offset, line) in base.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => {
                        let transformed_char = if c == 'x' { ' ' } else { c };
                        colored_base.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Bg(termion::color::AnsiValue(240)),
                            termion::color::Fg(termion::color::AnsiValue(237)),
                            transformed_char,
                            termion::color::Bg(termion::color::Reset),
                            termion::color::Fg(termion::color::Reset),
                        ))
                    }
                }
            }
        }

        format!("{}{}{}", colored_base, colored_wax, colored_flame)
    }
}

impl Collectable for Candle {
    fn get_symbol(&self) -> String {
        format!(
            "{}{}{}",
            termion::color::Bg(BACKGROUND_COLOR),
            "🕯️",
            termion::color::Bg(termion::color::Reset)
        )
    }
}
