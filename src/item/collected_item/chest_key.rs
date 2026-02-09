use crate::{
    item::{Item, collected_item::Collectable},
    tui::background::BACKGROUND_COLOR,
};
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
              BBBBBBBBCB=B9BBBBBBBBBBBBBB      █                                
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
                    'B' => colored_key.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(242)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    // encode one password letter for `Terminal`
                    c @ ('C' | '=' | '9') => colored_key.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(242)),
                        termion::color::Fg(termion::color::AnsiValue(237)),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                    c => colored_key.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(242)),
                        termion::color::Bg(BACKGROUND_COLOR),
                        c,
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                }
            }
        }
        colored_key
    }
}

impl Collectable for ChestKey {
    fn get_symbol(&self) -> String {
        format!(
            "{}{}{}",
            termion::color::Bg(BACKGROUND_COLOR),
            "🗝️",
            termion::color::Bg(termion::color::Reset)
        )
    }
}
