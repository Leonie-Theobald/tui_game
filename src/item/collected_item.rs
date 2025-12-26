pub mod candle;
pub mod rusty_key;
pub mod treasure_map;

use crate::{
    item::{
        Item,
        collected_item::{candle::Candle, rusty_key::RustyKey, treasure_map::TreasureMap},
    },
    tui::{
        Drawable,
        pop_up::{Button, PopUp},
    },
};
use termion::color;

#[derive(Debug, PartialEq)]
pub struct CollectedItems {
    pub list: Vec<CollectableItem>,
    pub visible: bool,
}

impl CollectedItems {
    pub fn initiate() -> Self {
        Self {
            list: Vec::new(),
            visible: false,
        }
    }

    pub fn show_all_in_popup(&self) {
        let mut popup_collected_item = PopUp {
            options: &self.list,
            active_option_index: 0,
            buttons: vec![Button::Confirm],
            active_button_index: 0,
        };
        #[expect(
            unused_must_use,
            reason = "We don't need user's selection here. 
                    It is just the API to present the items to the user."
        )]
        popup_collected_item.get_user_selection();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CollectableItem {
    RustyKey(RustyKey),
    Candle(Candle),
    TreasureMap(TreasureMap),
}

macro_rules! method_on_inner_collected_item {
    ($value:expr, $method:ident) => {
        match $value {
            CollectableItem::RustyKey(inner) => inner.$method(),
            CollectableItem::Candle(inner) => inner.$method(),
            CollectableItem::TreasureMap(inner) => inner.$method(),
        }
    };
}

impl std::fmt::Display for CollectableItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //     let inner_item = self.get_inner_item();
        //     writeln!(
        //         f,
        //         "{} - {}",
        //         inner_item.get_name(),
        //         inner_item.get_description()
        //     )
        let name = method_on_inner_collected_item!(self, get_name);
        let description = method_on_inner_collected_item!(self, get_description);

        writeln!(f, "{} - {}", name, description)
    }
}

impl Drawable for CollectedItems {
    fn draw(&self) -> String {
        if !self.visible {
            return String::new();
        }
        // format each item line and find out longest item line
        let mut length_longest_item_line = 0;
        let mut joined_items = String::new();
        for (index, item) in self.list.iter().enumerate() {
            let line = format!(
                "{}{}{}{}",
                termion::cursor::Goto(4, 13 + index as u16),
                color::Bg(color::Red),
                method_on_inner_collected_item!(item, get_name),
                color::Bg(color::Reset),
            );
            joined_items.push_str(&line);

            let item_name = method_on_inner_collected_item!(item, get_name);
            if item_name.len() > length_longest_item_line {
                length_longest_item_line = item_name.len();
            }
        }

        // format background depending on number of quests and the longest quest line
        // line length depends on quests length
        let background_line = format!(
            "{}{}{}",
            color::Bg(color::Red),
            " ".repeat(length_longest_item_line + 4),
            color::Bg(color::Reset),
        );

        // height depends on number of quests
        let mut background = String::new();
        for index in 0..self.list.len() + 2 {
            background.push_str(&format!(
                "{}{}",
                termion::cursor::Goto(2, 12 + index as u16),
                background_line
            ))
        }

        // merge background and written quests
        format!("{}{}", background, joined_items)
    }
}
