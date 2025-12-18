use std::fmt::Debug;
use termion::color;

use crate::{
    item::{Items, RustyKey},
    tui::{Drawable, Orientation, Vector, grid::Grid},
};

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub name: &'static str,
    pub backpack: Vec<Items>,
    pub coordinates: Vector,
    pub orientation: Orientation,
}

impl Player {
    pub fn initiate() -> Self {
        let mut player = Player {
            name: "Leonie",
            backpack: Vec::new(),
            coordinates: Vector {
                horizontal: 15,
                vertical: 14,
            },
            orientation: Orientation::North,
        };

        // add available item to players backpack
        let rusty_key = Items::RustyKey(RustyKey);
        player.backpack.push(rusty_key);

        player
    }

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
