use crate::{
    item::{Item, collected_item::Collectable},
    tui::background::BACKGROUND_COLOR,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Marshmallow;

impl Item for Marshmallow {
    fn get_name(&self) -> &'static str {
        "Marshmallow"
    }

    fn get_description(&self) -> &'static str {
        "An unroasted, squishy marshmallow"
    }

    fn get_detail_view(&self) -> String {
        let uncolored_marshmallow = String::from(
            "
                                                         
                             _____________________       
                         ___/.....................\\___  
                        /.............................\\ 
                       |\\_..........................._/|
                       |..\\_________________________/..|
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                       |...............................| 
                        \\_..........................._/ 
                          \\_________________________/   
                                                         ",
        );

        let mut colored_marshmallow = String::new();
        for (vertical_offset, line) in uncolored_marshmallow.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    '.' => colored_marshmallow.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(255)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => colored_marshmallow.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::LightBlack),
                        termion::color::Bg(termion::color::AnsiValue(255)),
                        c,
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                }
            }
        }

        colored_marshmallow
    }
}

impl Collectable for Marshmallow {
    fn get_symbol(&self) -> String {
        format!(
            "{}{}{}",
            termion::color::Bg(BACKGROUND_COLOR),
            "🖱️",
            termion::color::Bg(termion::color::Reset)
        )
    }
}
