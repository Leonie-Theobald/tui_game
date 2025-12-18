use std::io::{Write, stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let grid = Grid::import_from_string(
        "┌──────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│                                                                              │
│         ┌───────────────────────────┬──────────────────────┐                 │
│         │                           │                      │                 │
│         │                           │                      │                 │
│         │                                                  │                 │
│         │                                                  │                 │
│         │                                                  │                 │
│         │                           │                      │                 │
│         │                           │                      │                 │
│         ├──────────   ───────┬──────┴────────   ───────────┤                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         │                                                  │                 │
│         │                                                  │                 │
│         │                                                  │                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         │                    │                             │                 │
│         └────────────────────┴─────────────────────────────┘                 │
└──────────────────────────────────────────────────────────────────────────────┘",
    );

    let mut avatar = Avatar {
        grid: grid.clone(),
        coordinates: Vector {
            horizontal: 17,
            vertical: 15,
        },
        orientation: Orientation::East,
    };

    let mut canvas = Canvas {
        grids: vec![grid.clone(), avatar.map_into_grid()],
    };

    let mut stdout = stdout().into_raw_mode().unwrap();
    for k in stdin().keys() {
        let orientation = match k.as_ref().unwrap() {
            Key::Char('a') => Orientation::West,
            Key::Char('w') => Orientation::North,
            Key::Char('d') => Orientation::East,
            Key::Char('s') => Orientation::South,
            Key::Ctrl('c') => return,
            _ => {
                write!(stdout, "\r\nUnknown character... Exiting\r\n").unwrap();
                break;
            }
        };

        avatar.move_in_direction(orientation);
        canvas.grids[1] = avatar.map_into_grid();
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            canvas.visualize()
        )
        .unwrap();
        stdout.flush().unwrap();
    }
    stdout.suspend_raw_mode().unwrap();
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Vector {
    horizontal: usize,
    vertical: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
enum Orientation {
    #[default]
    North,
    East,
    South,
    West,
}

const GRID_WIDTH: usize = 80;
const GRID_HEIGHT: usize = 24;

#[derive(Clone, Debug)]
struct Grid {
    pixels: [[char; GRID_WIDTH]; GRID_HEIGHT],
}

enum TuiError {
    PositionOutOfBound,
    PositionBlocked,
}

impl Grid {
    fn empty() -> Self {
        Self {
            pixels: [[' '; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    fn get_new_free_coordinates(
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

    fn import_from_string(str: &'static str) -> Self {
        let mut grid = Self::empty();

        for (index, line) in str.split('\n').enumerate() {
            assert_eq!(line.chars().count(), GRID_WIDTH);
            assert!(index < GRID_HEIGHT);

            let transformed_line = line.chars().collect::<Vec<_>>();

            grid.pixels[index].copy_from_slice(&transformed_line[..]);
        }

        grid
    }

    fn draw(&self) -> String {
        //     let joined_lines = self.pixels.join(&['\n'][..]);
        //     joined_lines.iter().collect()
        self.pixels
            .iter()
            .map(|row| row.iter().collect())
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}

#[derive(Debug)]
struct Canvas {
    grids: Vec<Grid>,
}

impl Canvas {
    fn merge_grids(&self) -> Grid {
        let mut grid = Grid::empty();

        // go through each object
        for object in &self.grids {
            // go through the lines
            for (line_index, line) in object.pixels.iter().enumerate() {
                // go through the chars in the line
                for (column_index, character) in line.iter().enumerate() {
                    // ultimately every character is checked
                    // a non-white space character overwrites the grid
                    if character != &' ' {
                        grid.pixels[line_index][column_index] = *character;
                    }
                }
            }
        }

        grid
    }

    fn visualize(&self) -> String {
        self.merge_grids().draw()
    }
}

struct Avatar {
    grid: Grid,
    coordinates: Vector,
    orientation: Orientation,
}

impl Avatar {
    fn move_in_direction(&mut self, orientation: Orientation) {
        // avatar rotates to face requested direction
        self.orientation = orientation.clone();

        // check if avatar can make one step
        if let Ok(new_coordinates) = self
            .grid
            .get_new_free_coordinates(&self.coordinates, &orientation)
        {
            self.coordinates = new_coordinates;
        }
    }

    fn map_into_grid(&self) -> Grid {
        let mut grid = Grid::empty();
        let symbol = match self.orientation {
            Orientation::North => '▲',
            Orientation::East => '▶',
            Orientation::South => '▼',
            Orientation::West => '◀',
        };
        grid.pixels[self.coordinates.vertical][self.coordinates.horizontal] = symbol;

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_right_unblocked() {
        let start_coordinates = Vector::default();
        let start_orientation = Orientation::North;
        let mut avatar = Avatar {
            grid: Grid::empty(),
            coordinates: start_coordinates,
            orientation: start_orientation,
        };

        avatar.move_in_direction(Orientation::East);

        let end_coordinates = Vector {
            horizontal: 1,
            vertical: 0,
        };
        let end_orientation = Orientation::East;

        assert_eq!(avatar.coordinates, end_coordinates);
        assert_eq!(avatar.orientation, end_orientation);
    }

    #[test]
    fn move_down_blocked_by_wall() {
        let mut grid = Grid::empty();
        grid.pixels[1][0] = 'x';

        let start_coordinates = Vector::default();
        let start_orientation = Orientation::North;
        let mut avatar = Avatar {
            grid,
            coordinates: start_coordinates,
            orientation: start_orientation,
        };

        avatar.move_in_direction(Orientation::South);

        let end_coordinates = Vector::default();
        let end_orientation = Orientation::South;

        assert_eq!(avatar.coordinates, end_coordinates);
        assert_eq!(avatar.orientation, end_orientation);
    }

    #[test]
    fn move_in_circle() {
        let start_coordinates = Vector::default();
        let start_orientation = Orientation::North;

        let mut avatar = Avatar {
            grid: Grid::empty(),
            coordinates: start_coordinates,
            orientation: start_orientation,
        };

        avatar.move_in_direction(Orientation::East);
        avatar.move_in_direction(Orientation::South);
        avatar.move_in_direction(Orientation::West);
        avatar.move_in_direction(Orientation::North);

        let end_coordinates = Vector::default();
        let end_orientation = Orientation::North;

        assert_eq!(avatar.coordinates, end_coordinates);
        assert_eq!(avatar.orientation, end_orientation);
    }
}
