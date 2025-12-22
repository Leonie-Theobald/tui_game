pub mod grid;
pub mod pop_up;

pub enum TuiError {
    PositionOutOfBound,
    PositionBlocked,
    NoSelectionOptions,
    NoSelection,
}

pub trait Drawable {
    fn draw(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Vector {
    pub horizontal: usize,
    pub vertical: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Orientation {
    #[default]
    North,
    East,
    South,
    West,
}
