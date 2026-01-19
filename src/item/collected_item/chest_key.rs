use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChestKey;

impl Item for ChestKey {
    fn get_name(&self) -> &'static str {
        "Chest Key"
    }

    fn get_description(&self) -> &'static str {
        "This key belongs to a treasure chest."
    }

    fn get_detail_view(&self) -> String {
        let uncolored_key = "
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                          ████                                  
                                         █    █                                
                                        █      █                                
                                        █      █                                
              ███████████████████████████      █                                
              █ █                       █      █                                
                ▘                       █      █                                
                                         █    █                                 
                                          ████                                  
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                ";
        let mut colored_key = String::new();
        for (vertical_offset, line) in uncolored_key.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => colored_key.push_str(&format!(
                        "{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(242)),
                        c,
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }
        colored_key
    }
}

impl Collectable for ChestKey {
    fn get_symbol(&self) -> String {
        String::from("🗝️")
    }
}
