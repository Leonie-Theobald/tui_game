use crate::{
    item::{
        Item,
        collected_item::CollectedItems,
        stationed_item::{
            Stationary,
            interaction::{Interaction, InteractionStatus},
        },
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 55,
    vertical: 19,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LostChestKey {}

impl LostChestKey {
    pub fn initiate() -> Self {
        Self {}
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

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        todo!()
    }

    fn interact(&mut self, _collected_items: &mut CollectedItems) {
        todo!()
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        todo!()
    }

    fn set_interaction_status_to_finished(&mut self) {
        todo!()
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
