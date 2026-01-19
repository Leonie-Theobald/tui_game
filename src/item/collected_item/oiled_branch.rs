use crate::item::{
    Item,
    collected_item::{Collectable, branch::Branch},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OiledBranch;

impl Item for OiledBranch {
    fn get_name(&self) -> &'static str {
        "Oiled Branch"
    }

    fn get_description(&self) -> &'static str {
        "A straight branch dipped in oil."
    }

    fn get_detail_view(&self) -> String {
        let oiled_part = " 
 
 
                         _          
                        //
                       //
                      //
                -----/.__/
                    /./  \\   
                   /./___";

        let mut colored_oiled_part = String::new();
        for (vertical_offset, line) in oiled_part.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => {
                        let transformed_char = if c == '.' { ' ' } else { c };

                        colored_oiled_part.push_str(&format!(
                            "{}{}{}{}{}{}",
                            termion::cursor::Goto(
                                (horizontal_offset + 1) as u16,
                                (vertical_offset + 1) as u16
                            ),
                            termion::color::Fg(termion::color::AnsiValue(237)),
                            termion::color::Bg(termion::color::AnsiValue(239)),
                            transformed_char,
                            termion::color::Fg(termion::color::Reset),
                            termion::color::Bg(termion::color::Reset),
                        ))
                    }
                }
            }
        }

        let colored_branch = Branch.get_detail_view();

        format!("{}{}", colored_branch, colored_oiled_part)
    }
}

impl Collectable for OiledBranch {
    fn get_symbol(&self) -> String {
        format!(
            "{}{}{}",
            termion::color::Bg(termion::color::AnsiValue(234)),
            "🪾",
            termion::color::Bg(termion::color::Reset)
        )
    }
}
