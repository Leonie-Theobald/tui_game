use crate::{
    item::{Item, stationed_item::Stationary},
    quest::{Quest, collect_chest_key::CollectChestKey},
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 55,
    vertical: 19,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LostChestKey {
    linked_quest: Quest,
}

impl LostChestKey {
    pub fn initiate() -> Self {
        let linked_quest = Quest::CollectChestKey(CollectChestKey);
        Self { linked_quest }
    }
}

impl Item for LostChestKey {
    fn get_name(&self) -> &'static str {
        "Lost Chest Key"
    }

    fn get_description(&self) -> &'static str {
        "Somebody lost this key. It belongs to a treasure chest"
    }
}

impl Stationary for LostChestKey {
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

impl Drawable for LostChestKey {
    fn draw(&self) -> String {
        let symbol = "🗝️";

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
