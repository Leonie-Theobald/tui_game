use crate::{
    item::{Item, collected_item::Collectable},
    tui::background::BACKGROUND_COLOR,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Branch;

impl Item for Branch {
    fn get_name(&self) -> &'static str {
        "Branch"
    }

    fn get_description(&self) -> &'static str {
        "A dry and straight branch"
    }

    fn get_detail_view(&self) -> String {
        let uncolored_branch = "


                         _          
                        //
                       //
                      //
                -----/ __/
                    / /  \\   
                   / /___   
              \\___/ /__        
                /  /   _ 
              _/  /___/___/    
            _/  _/    \\ 
         __/  _/ 
        \\___/";

        let mut colored_branch = String::new();
        for (vertical_offset, line) in uncolored_branch.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    c => colored_branch.push_str(&format!(
                        "{}{}{}{}{}{}",
                        termion::cursor::Goto(
                            (horizontal_offset + 1) as u16,
                            (vertical_offset + 1) as u16
                        ),
                        termion::color::Fg(termion::color::AnsiValue(130)),
                        termion::color::Bg(BACKGROUND_COLOR),
                        c,
                        termion::color::Fg(termion::color::Reset),
                        termion::color::Bg(termion::color::Reset),
                    )),
                }
            }
        }

        colored_branch
    }
}

impl Collectable for Branch {
    fn get_symbol(&self) -> String {
        format!(
            "{}{}{}",
            termion::color::Bg(BACKGROUND_COLOR),
            "🪾",
            termion::color::Bg(termion::color::Reset)
        )
    }
}
