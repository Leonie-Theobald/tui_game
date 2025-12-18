use std::io::{Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

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
        coordinates: Vector {
            horizontal: 17,
            vertical: 15,
        },
        orientation: Orientation::East,
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

        avatar.move_in_direction(&grid, orientation);
        write!(
            stdout,
            "{}{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            grid.draw(),
            termion::cursor::Goto(1, 1),
            avatar.draw_into_grid(),
            termion::cursor::Goto(1, 1),
        )
        .unwrap();
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::clear::All).unwrap();
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
        self.pixels
            .iter()
            .map(|row| row.iter().collect())
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}

struct Avatar {
    coordinates: Vector,
    orientation: Orientation,
}

impl Avatar {
    fn move_in_direction(&mut self, grid: &Grid, orientation: Orientation) {
        // avatar rotates to face requested direction
        self.orientation = orientation.clone();

        // check if avatar can make one step
        if let Ok(new_coordinates) = grid.get_new_free_coordinates(&self.coordinates, &orientation)
        {
            self.coordinates = new_coordinates;
        }
    }

    fn draw_into_grid(&self) -> String {
        let symbol = match self.orientation {
            Orientation::North => '▲',
            Orientation::East => '▶',
            Orientation::South => '▼',
            Orientation::West => '◀',
        };
        format!(
            "{}{}",
            termion::cursor::Goto(
                self.coordinates.horizontal as u16,
                self.coordinates.vertical as u16
            ),
            symbol
        )
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
            coordinates: start_coordinates,
            orientation: start_orientation,
        };

        avatar.move_in_direction(&Grid::empty(), Orientation::East);

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
            coordinates: start_coordinates,
            orientation: start_orientation,
        };

        avatar.move_in_direction(&grid, Orientation::South);

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
            coordinates: start_coordinates,
            orientation: start_orientation,
        };

        avatar.move_in_direction(&Grid::empty(), Orientation::East);
        avatar.move_in_direction(&Grid::empty(), Orientation::South);
        avatar.move_in_direction(&Grid::empty(), Orientation::West);
        avatar.move_in_direction(&Grid::empty(), Orientation::North);

        let end_coordinates = Vector::default();
        let end_orientation = Orientation::North;

        assert_eq!(avatar.coordinates, end_coordinates);
        assert_eq!(avatar.orientation, end_orientation);
    }
}
