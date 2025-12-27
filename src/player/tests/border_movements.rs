/// Test whether the player ends up in the right position after trying to move in the direction of
/// the border while already at it.
/// Only the orientation might change
#[cfg(test)]
mod tests {
    use crate::{
        item::stationed_item::StationedItems,
        player::Player,
        tui::{
            Orientation, Vector,
            grid::{GRID_HEIGHT, GRID_WIDTH, Grid},
        },
    };

    #[test]
    fn move_left_on_left_border() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 0,
                vertical: 2,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 0,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::West);

        assert_eq!(target_player, player)
    }

    #[test]
    fn move_right_on_right_border() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: GRID_WIDTH - 1,
                vertical: 2,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: GRID_WIDTH - 1,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::East);

        assert_eq!(target_player, player)
    }

    #[test]
    fn move_top_on_top_border() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 5,
                vertical: 0,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 5,
                vertical: 0,
            },
            orientation: Orientation::North,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::North);

        assert_eq!(target_player, player)
    }

    #[test]
    fn move_down_on_bottom_border() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 5,
                vertical: GRID_HEIGHT - 1,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 5,
                vertical: GRID_HEIGHT - 1,
            },
            orientation: Orientation::South,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::South);

        assert_eq!(target_player, player)
    }
}
