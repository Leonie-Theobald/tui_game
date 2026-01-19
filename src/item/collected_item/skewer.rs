use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skewer;

impl Item for Skewer {
    fn get_name(&self) -> &'static str {
        "Skewer"
    }

    fn get_description(&self) -> &'static str {
        "A small piece of wood"
    }

    fn get_detail_view(&self) -> String {
        let uncolored_skewer = "


                         /         
                        /
                       /
                      /
                     /";

        let mut colored_skewer = String::new();
        for (vertical_offset, line) in uncolored_skewer.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => colored_skewer.push_str(&format!(
                        "{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(130)),
                        c,
                        termion::color::Fg(termion::color::Reset),
                    )),
                }
            }
        }

        colored_skewer
    }
}

impl Collectable for Skewer {
    fn get_symbol(&self) -> String {
        format!(
            "{}/{}",
            termion::color::Fg(termion::color::AnsiValue(130)),
            termion::color::Fg(termion::color::Reset)
        )
    }
}
