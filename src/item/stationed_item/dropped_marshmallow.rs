use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            dropped_marshmallow_interaction::{DroppedMarshmallowInteraction, SkewerMarshmallow},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 16,
    vertical: 12,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DroppedMarshmallow {
    picked_up: bool,
    linked_interactions: Vec<Interaction>,
}

impl DroppedMarshmallow {
    pub fn initiate() -> Self {
        let linked_interactions = vec![Interaction::DroppedMarshmallow(
            DroppedMarshmallowInteraction::SkewerMarshmallow(SkewerMarshmallow),
        )];
        Self {
            picked_up: false,
            linked_interactions,
        }
    }
}

impl Item for DroppedMarshmallow {
    fn get_name(&self) -> &'static str {
        "Dropped Marshmallow"
    }

    fn get_description(&self) -> &'static str {
        "There is a marshmallow on the ground."
    }

    fn get_detail_view(&self) -> String {
        let plain_marshmallow = String::from(
            "
                                                          \r
                             _____________________        \r
                         ___/.....................\\___   \r
                        /.............................\\  \r
                       |\\_..........................._/| \r
                       |..\\_________________________/..| \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                        \\_..........................._/  \r
                          \\_________________________/    \r
                                                         ",
        );

        plain_marshmallow
            .chars()
            .fold(String::new(), |mut white_marshmallow, c| {
                if c.is_whitespace() {
                    white_marshmallow.push(c);
                } else {
                    white_marshmallow.push_str(&format!(
                        "{}{}{}{}{}",
                        termion::color::Bg(termion::color::White),
                        termion::color::Fg(termion::color::LightBlack),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset)
                    ))
                }
                white_marshmallow
            })
            .replace(".", " ")
    }
}

impl Stationary for DroppedMarshmallow {
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
        if self.picked_up {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.picked_up = true;
    }
}

impl Drawable for DroppedMarshmallow {
    fn draw(&self) -> String {
        let symbol = if self.picked_up { "X" } else { "🖱️" };

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
