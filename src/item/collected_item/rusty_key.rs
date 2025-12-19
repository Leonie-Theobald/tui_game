use crate::item::Item;

#[derive(Clone, Debug, PartialEq)]
pub struct RustyKey;

impl Item for RustyKey {
    fn get_name(&self) -> &'static str {
        "Rusty Key"
    }

    fn get_description(&self) -> &'static str {
        "An old, rusty key with a mysterious engraving."
    }
}
