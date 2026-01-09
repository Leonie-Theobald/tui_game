use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        stationed_item::{
            Stationary,
            interaction::{Interaction, InteractionStatus, PickUpChestKey},
        },
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 55,
    vertical: 19,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LostChestKey {
    found: bool,
    linked_interactions: Vec<Interaction>,
}

impl LostChestKey {
    pub fn initiate() -> Self {
        let linked_interactions = vec![Interaction::PickUpChestKey(PickUpChestKey {})];
        Self {
            found: false,
            linked_interactions,
        }
    }
}

impl Item for LostChestKey {
    fn get_name(&self) -> &'static str {
        "Lost Chest Key"
    }

    fn get_description(&self) -> &'static str {
        "Somebody lost this key. It belongs to a treasure chest"
    }

    fn get_detail_view(&self) -> String {
        todo!()
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
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        if self.found {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.found = true;
    }
}

impl Drawable for LostChestKey {
    fn draw(&self) -> String {
        let symbol = if self.found { "X" } else { "🗝️" };

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
