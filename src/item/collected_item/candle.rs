use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Candle;

impl Item for Candle {
    fn get_name(&self) -> &'static str {
        "Candle"
    }

    fn get_description(&self) -> &'static str {
        "A candle that is not burning."
    }
}

impl Collectable for Candle {
    fn get_symbol(&self) -> String {
        String::from("🕯️")
    }
}
