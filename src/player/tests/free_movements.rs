/// Test whether the player ends up in the right position after moving one step without any
/// blockages
#[cfg(test)]
mod tests {
    use crate::{
        item::stationed_item::StationedItems,
        player::Player,
        tui::{Orientation, Vector, grid::Grid},
    };

    #[test]
    fn north_faced_player_moves_north() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 1,
            },
            orientation: Orientation::North,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::North);

        assert_eq!(target_player, player)
    }

    #[test]
    fn north_faced_player_moves_east() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 3,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::East);

        assert_eq!(target_player, player)
    }

    #[test]
    fn north_faced_player_moves_south() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 3,
            },
            orientation: Orientation::South,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::South);

        assert_eq!(target_player, player)
    }

    #[test]
    fn north_faced_player_moves_west() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 1,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::West);

        assert_eq!(target_player, player)
    }

    #[test]
    fn east_faced_player_moves_north() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 1,
            },
            orientation: Orientation::North,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::North);

        assert_eq!(target_player, player)
    }

    #[test]
    fn east_faced_player_moves_east() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 3,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::East);

        assert_eq!(target_player, player)
    }

    #[test]
    fn east_faced_player_moves_south() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 3,
            },
            orientation: Orientation::South,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::South);

        assert_eq!(target_player, player)
    }

    #[test]
    fn east_faced_player_moves_west() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 1,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::West);

        assert_eq!(target_player, player)
    }

    #[test]
    fn south_faced_player_moves_north() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::South,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 1,
            },
            orientation: Orientation::North,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::North);

        assert_eq!(target_player, player)
    }

    #[test]
    fn south_faced_player_moves_east() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::South,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 3,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::East);

        assert_eq!(target_player, player)
    }

    #[test]
    fn south_faced_player_moves_south() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::South,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 3,
            },
            orientation: Orientation::South,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::South);

        assert_eq!(target_player, player)
    }

    #[test]
    fn south_faced_player_moves_west() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::South,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 1,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::West);

        assert_eq!(target_player, player)
    }

    #[test]
    fn west_faced_player_moves_north() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 1,
            },
            orientation: Orientation::North,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::North);

        assert_eq!(target_player, player)
    }

    #[test]
    fn west_faced_player_moves_east() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 3,
                vertical: 2,
            },
            orientation: Orientation::East,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::East);

        assert_eq!(target_player, player)
    }

    #[test]
    fn west_faced_player_moves_south() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 3,
            },
            orientation: Orientation::South,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::South);

        assert_eq!(target_player, player)
    }

    #[test]
    fn west_faced_player_moves_west() {
        let grid = Grid::empty();
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 2,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 1,
                vertical: 2,
            },
            orientation: Orientation::West,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::West);

        assert_eq!(target_player, player)
    }
}
