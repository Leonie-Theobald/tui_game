use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        collected_item::{CollectableItem, CollectedItems, chest_key::ChestKey},
        interaction::{Interactable, Interaction, InteractionStatus},
        stationed_item::Stationary,
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
        let linked_interactions = vec![Interaction::LostChestKey(LostChestKeyInteraction::PickUp(
            PickUp {},
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LostChestKeyInteraction {
    PickUp(PickUp),
}

impl LostChestKeyInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            LostChestKeyInteraction::PickUp(pick_up) => pick_up,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PickUp;
impl Interactable for PickUp {
    fn get_prompt(&self) -> String {
        String::from("Take the lost key")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                collected_items
                    .list
                    .push(CollectableItem::ChestKey(ChestKey));
                linked_stationary_item.set_interaction_status_to_finished();
                String::from("Hurray! You found something.")
            }
            InteractionStatus::Finished => String::from("Nothing there"),
        }
    }
}
