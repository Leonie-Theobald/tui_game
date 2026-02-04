use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            pressure_plate_interaction::{Press, PressurePlateInteraction},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 44,
    vertical: 5,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PressurePlate {
    pub pressed: bool,
    linked_interactions: Vec<Interaction>,
}

impl PressurePlate {
    pub fn initiate() -> Self {
        let linked_interactions = vec![Interaction::PressurePlate(
            PressurePlateInteraction::Press(Press),
        )];

        Self {
            pressed: false,
            linked_interactions,
        }
    }
}

impl Item for PressurePlate {
    fn get_name(&self) -> &'static str {
        "Pressure Plate"
    }

    fn get_description(&self) -> &'static str {
        "A small plate in the wall that can be pressed."
    }

    fn get_detail_view(&self) -> String {
        String::new()
    }
}

impl Stationary for PressurePlate {
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
        if self.pressed {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.pressed = true;
    }
}

impl Drawable for PressurePlate {
    fn draw(&self) -> String {
        format!(
            "{}│",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
        )
    }
}
