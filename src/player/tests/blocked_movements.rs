/// Test whether the player ends up in the right position if the way is blocked by grid elements or
/// stationary items
#[cfg(test)]
mod tests {
    use crate::{
        item::stationed_item::{StationaryItem, StationedItems, packet::Packet},
        player::Player,
        tui::{Orientation, Vector, grid::Grid},
    };

    #[test]
    fn move_against_grid_obstacle() {
        // grid where top left corner is blocked
        let mut grid = Grid::empty();
        grid.pixels[0][0] = 'x';
        let stationed_items = StationedItems { list: Vec::new() };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 1,
                vertical: 0,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 1,
                vertical: 0,
            },
            orientation: Orientation::West,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::West);

        assert_eq!(target_player, player)
    }

    #[test]
    fn move_against_stationary_item() {
        let grid = Grid::empty();
        let stationed_items = StationedItems {
            list: vec![StationaryItem::Packet(Packet::initiate())],
        };
        let mut player = Player {
            coordinates: Vector {
                horizontal: 15,
                vertical: 6,
            },
            orientation: Orientation::North,
        };

        let target_player = Player {
            coordinates: Vector {
                horizontal: 15,
                vertical: 6,
            },
            orientation: Orientation::South,
        };

        player.move_in_direction(&grid, &stationed_items, Orientation::South);

        assert_eq!(target_player, player)
    }
}
