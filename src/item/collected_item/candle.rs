use crate::item::{Item, collected_item::Collectable};

#[derive(Clone, Debug, PartialEq)]
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
