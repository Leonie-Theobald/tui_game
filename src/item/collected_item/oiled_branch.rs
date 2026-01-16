use crate::item::{Item, collected_item::Collectable};
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
        let branch = String::from(
            "


                         _          
                        //
                       //
                      //
                _____/.__
                    /./ \\   
                   /./___/   
             \\___/ /_ \\         
                /  /   _ 
              _/  /___/___/    
            _/  _/   \\ 
        ___/ __/ 
        \\___/",
        );

        let mut oiled_branch = String::new();
        for (index, line) in branch.lines().enumerate() {
            if index < 10 {
                // part covered with oil
                for char in line.chars() {
                    if char.is_whitespace() {
                        oiled_branch.push(char);
                    } else {
                        oiled_branch.push_str(&format!(
                            "{}{}{}",
                            termion::color::Bg(termion::color::AnsiValue(234)),
                            char,
                            termion::color::Bg(termion::color::Reset),
                        ))
                    }
                }
            } else {
                oiled_branch.push_str(line);
            }
            oiled_branch.push_str("\r\n");
        }

        oiled_branch
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
