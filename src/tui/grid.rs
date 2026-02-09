use crate::tui::{Drawable, Orientation, TuiError, Vector, background::BACKGROUND_COLOR};

pub const GRID_WIDTH: usize = 80;
pub const GRID_HEIGHT: usize = 22;

#[derive(Clone, Debug)]
pub struct Grid {
    pub pixels: [[char; GRID_WIDTH]; GRID_HEIGHT],
}

impl Grid {
    pub fn empty() -> Self {
        Self {
            pixels: [[' '; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    pub fn get_new_free_coordinates(
        &self,
        coordinates: &Vector,
        orientation: &Orientation,
    ) -> Result<Vector, TuiError> {
        // check general bounds
        let is_furthest_north = coordinates.vertical == 0;
        let is_furthest_west = coordinates.horizontal == 0;
        let is_furthest_south = coordinates.vertical == GRID_HEIGHT - 1;
        let is_furthest_east = coordinates.horizontal == GRID_WIDTH - 1;

        // if position is at edge and it goes one step further in this direction,
        // the movement would not be within grid
        if is_furthest_north && orientation == &Orientation::North {
            return Err(TuiError::PositionOutOfBound);
        }
        if is_furthest_west && orientation == &Orientation::West {
            return Err(TuiError::PositionOutOfBound);
        }
        if is_furthest_south && orientation == &Orientation::South {
            return Err(TuiError::PositionOutOfBound);
        }
        if is_furthest_east && orientation == &Orientation::East {
            return Err(TuiError::PositionOutOfBound);
        }

        let mut new_coordinates = coordinates.clone();
        // make a step
        match orientation {
            Orientation::North => new_coordinates.vertical -= 1,
            Orientation::East => new_coordinates.horizontal += 1,
            Orientation::South => new_coordinates.vertical += 1,
            Orientation::West => new_coordinates.horizontal -= 1,
        };

        // if the pixel is white space then it is a free pixel
        let pixel = self.pixels[new_coordinates.vertical][new_coordinates.horizontal];
        match pixel {
            ' ' => Ok(new_coordinates),
            _ => Err(TuiError::PositionBlocked),
        }
    }

    pub fn import_from_string(str: &'static str) -> Self {
        let mut grid = Self::empty();

        for (index, line) in str.split('\n').enumerate() {
            assert_eq!(line.chars().count(), GRID_WIDTH);
            assert!(index < GRID_HEIGHT);

            let transformed_line = line.chars().collect::<Vec<_>>();

            grid.pixels[index].copy_from_slice(&transformed_line[..]);
        }

        grid
    }

    pub fn initiate() -> Self {
        Self::import_from_string(
            "┌──────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│                                                                              │
│         ┌──────────────────┐                                                 │
│         │                  │                                                 │
│         │                  │                                                 │
│         │                  │                                                 │
│         └──────────   ─────┘                                                 │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘",
        )
    }
}

impl Drawable for Grid {
    fn draw(&self) -> String {
        let mut colored_grid = String::new();
        for (vertical_offset, line) in self.pixels.iter().enumerate() {
            for (horizontal_offset, char) in line.iter().enumerate() {
                colored_grid.push_str(&format!(
                    "{}{}{}{}",
                    termion::cursor::Goto(
                        (horizontal_offset + 1) as u16,
                        (vertical_offset + 1) as u16
                    ),
                    termion::color::Bg(BACKGROUND_COLOR),
                    char,
                    termion::color::Bg(termion::color::Reset),
                ));
            }
        }

        colored_grid
    }
}
