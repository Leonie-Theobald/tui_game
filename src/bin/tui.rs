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
        current_grid: grid.clone(),
        current_position: Position {
            coordinates: Vector {
                horizontal: 17,
                vertical: 15,
            },
            orientation: Orientation::East,
        },
    };

    avatar.move_in_direction(Orientation::North);
    avatar.move_in_direction(Orientation::North);
    avatar.move_in_direction(Orientation::East);
    avatar.move_in_direction(Orientation::East);
    avatar.move_in_direction(Orientation::East);
    avatar.move_in_direction(Orientation::East);
    avatar.move_in_direction(Orientation::North);
    avatar.move_in_direction(Orientation::North);

    let canvas = Canvas {
        grids: vec![grid, avatar.map_into_grid()],
    };

    println!("Grid:\n{}", canvas.visualize());
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Vector {
    horizontal: usize,
    vertical: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Position {
    coordinates: Vector,
    orientation: Orientation,
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

    fn get_new_free_position(
        &self,
        coordinates: &Vector,
        orientation: &Orientation,
    ) -> Result<Position, TuiError> {
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
            ' ' => Ok(Position {
                coordinates: new_coordinates,
                orientation: orientation.clone(),
            }),
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
        let joined_lines = self.pixels.join(&['\n'][..]);
        joined_lines.iter().collect()
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
    current_grid: Grid,
    current_position: Position,
}

impl Avatar {
    fn move_in_direction(&mut self, orientation: Orientation) {
        // avatar rotates to face requested direction
        self.current_position.orientation = orientation.clone();

        // check if avatar can make one step
        if let Ok(new_position) = self
            .current_grid
            .get_new_free_position(&self.current_position.coordinates, &orientation)
        {
            self.current_position = new_position;
        }
    }

    fn map_into_grid(&self) -> Grid {
        let mut grid = Grid::empty();
        let symbol = match self.current_position.orientation {
            Orientation::North => '▲',
            Orientation::East => '▶',
            Orientation::South => '▼',
            Orientation::West => '◀',
        };
        grid.pixels[self.current_position.coordinates.vertical]
            [self.current_position.coordinates.horizontal] = symbol;

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_right_unblocked() {
        let start_position = Position::default();
        let mut avatar = Avatar {
            current_grid: Grid::empty(),
            current_position: start_position,
        };

        avatar.move_in_direction(Orientation::East);

        let end_position = Position {
            coordinates: Vector {
                horizontal: 1,
                vertical: 0,
            },
            orientation: Orientation::East,
        };

        assert_eq!(avatar.current_position, end_position);
    }

    #[test]
    fn move_down_blocked_by_wall() {
        let mut grid = Grid::empty();
        grid.pixels[1][0] = 'x';

        let start_position = Position::default();
        let mut avatar = Avatar {
            current_grid: grid,
            current_position: start_position,
        };

        avatar.move_in_direction(Orientation::South);

        let end_position = Position {
            orientation: Orientation::South,
            ..Default::default()
        };

        assert_eq!(avatar.current_position, end_position);
    }

    #[test]
    fn move_in_circle() {
        let start_position = Position {
            ..Default::default()
        };
        let mut avatar = Avatar {
            current_grid: Grid::empty(),
            current_position: start_position,
        };

        avatar.move_in_direction(Orientation::East);
        avatar.move_in_direction(Orientation::South);
        avatar.move_in_direction(Orientation::West);
        avatar.move_in_direction(Orientation::North);

        let end_position = Position::default();

        assert_eq!(avatar.current_position, end_position);
    }
}
