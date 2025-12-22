use std::fmt::Debug;
use termion::color;

use crate::{
    item::stationed_item::{Stationary, StationaryItem, StationedItems},
    tui::{Drawable, Orientation, Vector, grid::Grid},
};

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

    pub fn move_in_direction(
        &mut self,
        grid: &Grid,
        stationed_items: &StationedItems,
        orientation: Orientation,
    ) {
        // avatar rotates to face requested direction
        self.orientation = orientation.clone();

        // merge blocking fields of stationed items
        let mut merged_grid = grid.clone();
        for stationed_item in &stationed_items.list {
            let blocked_pixels = match stationed_item {
                StationaryItem::Packet(packet) => packet.list_blocking_coordinates(),
                StationaryItem::TreasureChest(treasure_chest) => {
                    treasure_chest.list_blocking_coordinates()
                }
            };
            for blocked_pixel in blocked_pixels {
                // assign placeholder blocking element
                merged_grid.pixels[blocked_pixel.vertical][blocked_pixel.horizontal] = 'x';
            }
        }

        // check if avatar can make one step
        if let Ok(new_coordinates) =
            merged_grid.get_new_free_coordinates(&self.coordinates, &orientation)
        {
            self.coordinates = new_coordinates;
        }
    }

    /// The position the player is facing for interactions
    /// Right in front of the player
    pub fn get_facing_coordinates(&self) -> Vector {
        let mut target_coordinates = self.coordinates.clone();
        match self.orientation {
            Orientation::North => target_coordinates.vertical -= 1,
            Orientation::East => target_coordinates.horizontal += 1,
            Orientation::South => target_coordinates.vertical -= 1,
            Orientation::West => target_coordinates.horizontal -= 1,
        };

        target_coordinates
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
