pub mod grid;
pub mod pop_up;
pub mod status_bar;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TuiError {
    PositionOutOfBound,
    PositionBlocked,
    NoSelection,
    NoSelectionOptions,
}

pub trait Drawable {
    fn draw(&self) -> String;
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    pub horizontal: usize,
    pub vertical: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Orientation {
    #[default]
    North,
    East,
    South,
    West,
}
