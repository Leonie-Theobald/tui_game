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
        todo!()
    }
}

impl Collectable for Torch {
    fn get_symbol(&self) -> String {
        String::from("🔥")
    }
}
