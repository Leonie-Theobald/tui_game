use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        collected_item::chest_key::ChestKey,
        interaction::{
            Interaction, InteractionStatus,
            lost_chest_key_interaction::{LostChestKeyInteraction, PickUp},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector, background::BACKGROUND_COLOR},
};

const COORDINATES: Vector = Vector {
    horizontal: 25,
    vertical: 5,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LostChestKey {
    found: bool,
    linked_interactions: Vec<Interaction>,
}

impl LostChestKey {
    pub fn initiate() -> Self {
        let linked_interactions = vec![Interaction::LostChestKey(LostChestKeyInteraction::PickUp(
            PickUp,
        ))];
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
        ChestKey.get_detail_view()
    }
}

impl Stationary for LostChestKey {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        if self.found {
            return Vec::new();
        }

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
        let symbol = if self.found { "" } else { "🗝️" };

        format!(
            "{}{}{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            termion::color::Bg(BACKGROUND_COLOR),
            symbol,
            termion::color::Bg(termion::color::Reset),
        )
    }
}
