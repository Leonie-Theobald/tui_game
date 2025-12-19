use crate::item::Item;

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
