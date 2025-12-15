pub trait Item {
    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Items {
    RustyKey(RustyKey),
}

impl Items {
    pub fn get_inner_item(&self) -> impl Item {
        match self {
            Self::RustyKey(inner_item) => inner_item,
        }
    }
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner_item = self.get_inner_item();
        write!(
            f,
            "{} - {}",
            inner_item.get_name(),
            inner_item.get_description()
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RustyKey;

impl Item for &RustyKey {
    fn get_name(&self) -> &'static str {
        "Rust Key"
    }

    fn get_description(&self) -> &'static str {
        "An old, rusty key with a mysterious engraving."
    }
}
