use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Torch;

impl Item for Torch {
    fn get_name(&self) -> &'static str {
        "Torch"
    }

    fn get_description(&self) -> &'static str {
        "A burning torch"
    }

    fn get_detail_view(&self) -> String {
        let stick = "  
  
                                           
                                          
                                         
                                          
                                           
                                           
                                          
                                       
                                    ///
                                   ///
                                  /// 
                                 ///  
                                ///   
                               ///    
                              ///     
                             ///      
                            ///       
                           ///        
                          ///         
                         ///          
                        ///";
        let flames = "  
  
                                     . ' ,'
                                   ,/|\\'|
                                 \" ,|'/.
                                 #|(,'@\\)
                                 ||\\)||/\\
                                  \\@|.\\\\
                                   \\\\_/#
                                   ##/#";

        let mut colored_flames = String::new();
        for (vertical_offset, line) in flames.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => colored_flames.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(214)),
                        termion::color::Fg(termion::color::AnsiValue(166)),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }
        let mut colored_stick = String::new();
        for (vertical_offset, line) in stick.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => colored_stick.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(130)),
                        termion::color::Fg(termion::color::AnsiValue(94)),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }

        format!("{}{}", colored_stick, colored_flames)
    }
}

impl Collectable for Torch {
    fn get_symbol(&self) -> String {
        String::from("🔥")
    }
}
