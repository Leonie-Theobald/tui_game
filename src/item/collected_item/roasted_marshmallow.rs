use crate::{
    item::{Item, collected_item::Collectable},
    tui::background::BACKGROUND_COLOR,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoastedMarshmallow;

impl Item for RoastedMarshmallow {
    fn get_name(&self) -> &'static str {
        "Roasted Marshmallow"
    }

    fn get_description(&self) -> &'static str {
        "A roasted, crispy marshmallow"
    }

    fn get_detail_view(&self) -> String {
        let uncolored_marshmallow = String::from(
            "
                                                         
                             _____________________       
                         ___/...............bbbb..\\___  
                        /........bbbbb......bxxb......\\ 
                       |\\_......bxxx.......bbbb......_/|
                       |..\\_________________________/..|
                       |....bxxb.......bbbb............| 
                       |....bxxb.......bxxb............| 
                       |....bbbb.......bxxb............| 
                       |...............bbbb............| 
                       |...............................| 
                       |...............................| 
                       |.............bbbb..............| 
                       |.............bxxb..............| 
                       |...bbbb......bxxb..............| 
                       |...bxxb......bbbb......bbbb....| 
                       |...bxxb................bxxb....| 
                       |...bbbb................bxxb....| 
                       |.......................bbbb....| 
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
                        termion::color::Bg(termion::color::AnsiValue(215)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    'x' => colored_marshmallow.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(235)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    'b' => colored_marshmallow.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::AnsiValue(130)),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    c => colored_marshmallow.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::LightBlack),
                        termion::color::Bg(termion::color::AnsiValue(215)),
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

impl Collectable for RoastedMarshmallow {
    fn get_symbol(&self) -> String {
        format!(
            "{}{}{}",
            termion::color::Bg(BACKGROUND_COLOR),
            "🟫",
            termion::color::Bg(termion::color::Reset)
        )
    }
}
