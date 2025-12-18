use termion::color;

use crate::tui::Drawable;

pub trait Item {
    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
}

#[derive(PartialEq)]
pub struct CollectedItems {
    pub list: Vec<Items>,
    pub visible: bool,
}

impl CollectedItems {
    pub fn initiate() -> Self {
        Self {
            list: vec![Items::RustyKey(RustyKey)],
            visible: false,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
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
                item.get_inner_item().get_name(),
                color::Bg(color::Reset),
            );
            joined_items.push_str(&line);

            if item.get_inner_item().get_name().len() > length_longest_item_line {
                length_longest_item_line = item.get_inner_item().get_name().len();
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

#[derive(Clone, Debug, PartialEq)]
pub enum Items {
    RustyKey(RustyKey),
    Candle(Candle),
    TreasureMap(TreasureMap),
}

impl Items {
    pub fn get_inner_item(&self) -> &dyn Item {
        match self {
            Self::RustyKey(inner_item) => inner_item,
            Self::Candle(candle) => candle,
            Self::TreasureMap(treasure_map) => treasure_map,
        }
    }
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner_item = self.get_inner_item();
        writeln!(
            f,
            "{} - {}",
            inner_item.get_name(),
            inner_item.get_description()
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RustyKey;

impl Item for RustyKey {
    fn get_name(&self) -> &'static str {
        "Rust Key"
    }

    fn get_description(&self) -> &'static str {
        "An old, rusty key with a mysterious engraving."
    }
}

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

#[derive(Clone, Debug, PartialEq)]
pub struct TreasureMap;

impl Item for TreasureMap {
    fn get_name(&self) -> &'static str {
        "Treasure Map"
    }

    fn get_description(&self) -> &'static str {
        "An illegible map with several red X on it."
    }
}
