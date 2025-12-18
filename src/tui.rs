pub mod grid;

pub enum TuiError {
    PositionOutOfBound,
    PositionBlocked,
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
