use crate::item::{Item, collected_item::Collectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RustyKey;

impl Item for RustyKey {
    fn get_name(&self) -> &'static str {
        "Rusty Key"
    }

    fn get_description(&self) -> &'static str {
        "An old, rusty key with a mysterious engraving."
    }

    fn get_detail_view(&self) -> String {
        todo!()
    }
}

impl Collectable for RustyKey {
    fn get_symbol(&self) -> String {
        String::from("🔑")
    }
}
