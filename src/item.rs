pub trait Item {
    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
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
