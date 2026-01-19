use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RustyKey;

impl Item for RustyKey {
    fn get_name(&self) -> &'static str {
        "Rusty Key"
    }

    fn get_description(&self) -> &'static str {
        "An old, rusty key with a mysterious engraving."
    }

    fn get_detail_view(&self) -> String {
        let uncolored_key = "


                                  __________
                                _/xxxxxxxxxx\\_    
                               /xxxxxxx____xxx\\    
                             _/xxxxxxx/    \\xxx\\   
                            /xxxxxxxxx|     |xxx|  
                            |xxxxxxxxxx\\___/xxxx|  
                           /xxxxxxxxxxxxxxxxxxxx|  
                           |xxxxxxxxxxxxxxxxxxxx|  
                        ___/xxxxxxxxxxxxxxxxxxx/   
                    ___/xxxxxxxxxxxxxxxxxxxxxx/    
                ___/xxx__xxxx/\\______________/     
            ___/xxxxxxx\\ \\xx\\        
        ___/xxxxxxx/ \\_/  \\_/                         
       |xxxxxx__xxx\\                                
       |xxxxx/  \\__|                                
       |____|";

        let mut colored_key = String::new();
        for (vertical_offset, line) in uncolored_key.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => {
                        let transformed_char = if c == 'x' { ' ' } else { c };

                        colored_key.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Fg(termion::color::Black),
                            termion::color::Bg(termion::color::AnsiValue(220)),
                            transformed_char,
                            termion::color::Fg(termion::color::Reset),
                            termion::color::Bg(termion::color::Reset),
                        ))
                    }
                }
            }
        }
        colored_key
    }
}

impl Collectable for RustyKey {
    fn get_symbol(&self) -> String {
        String::from("🔑")
    }
}
