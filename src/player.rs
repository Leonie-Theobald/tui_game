use std::fmt::Debug;

use crate::item::Items;

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub backpack: Vec<Items>,
}

impl Player {
    pub fn show_name(&self) -> &'static str {
        self.name
    }

    pub fn show_backpack(&self) -> String {
        let mut backpack = String::new();
        for item in &self.backpack {
            backpack.push_str(&item.to_string())
        }

        backpack
    }
}
