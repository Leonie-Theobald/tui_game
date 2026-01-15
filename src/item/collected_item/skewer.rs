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
        String::from(
            "


                         /         
                        /
                       /
                      /
                     /",
        )
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
