use crate::{
    item::{Item, stationed_item::Stationary},
    quest::{Quest, unlock_treasure_chest::UnlockTreasureChest},
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 15,
    vertical: 6,
};

#[derive(Clone, Debug, PartialEq)]
pub struct TreasureChest {
    linked_quest: Quest,
}

impl TreasureChest {
    pub fn initiate() -> Self {
        let linked_quest = Quest::UnlockTreasureChest(UnlockTreasureChest {});
        Self { linked_quest }
    }
}

impl Item for TreasureChest {
    fn get_name(&self) -> &'static str {
        "Treasure Chest"
    }

    fn get_description(&self) -> &'static str {
        "A chest with mysterious symbols on it."
    }
}

impl Stationary for TreasureChest {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinate = Vec::new();

        // +-1 around the symbol is also blocked
        let horizontal_range = COORDINATES.horizontal..COORDINATES.horizontal + 2;
        for horizontal_index in horizontal_range {
            let vertical_range = COORDINATES.vertical..COORDINATES.vertical + 1;
            for vertical_index in vertical_range {
                blocked_coordinate.push(Vector {
                    horizontal: horizontal_index,
                    vertical: vertical_index,
                })
            }
        }

        blocked_coordinate
    }

    fn get_linked_quest(&self) -> &Quest {
        &self.linked_quest
    }
}

impl Drawable for TreasureChest {
    fn draw(&self) -> String {
        let symbol = "🧳";

        format!(
            "{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            symbol,
        )
    }
}
