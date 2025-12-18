use std::fmt::Debug;
use termion::color;

use crate::tui::{Drawable, Orientation, Vector, grid::Grid};

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub coordinates: Vector,
    pub orientation: Orientation,
}

impl Player {
    pub fn initiate() -> Self {
        Self {
            name: "Leonie",
            coordinates: Vector {
                horizontal: 15,
                vertical: 14,
            },
            orientation: Orientation::North,
        }
    }

    pub fn show_name(&self) -> &'static str {
        self.name
    }

    pub fn move_in_direction(&mut self, grid: &Grid, orientation: Orientation) {
        // avatar rotates to face requested direction
        self.orientation = orientation.clone();

        // check if avatar can make one step
        if let Ok(new_coordinates) = grid.get_new_free_coordinates(&self.coordinates, &orientation)
        {
            self.coordinates = new_coordinates;
        }
    }
}
impl Drawable for Player {
    fn draw(&self) -> String {
        let symbol = match self.orientation {
            Orientation::North => '▲',
            Orientation::East => '▶',
            Orientation::South => '▼',
            Orientation::West => '◀',
        };
        format!(
            "{}{}{}{}",
            termion::cursor::Goto(
                (self.coordinates.horizontal + 1) as u16,
                (self.coordinates.vertical + 1) as u16
            ),
            color::Fg(color::Magenta),
            symbol,
            color::Fg(color::Reset),
        )
    }
}
