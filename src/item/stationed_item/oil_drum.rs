use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            oil_drum_interaction::{DipBranch, OilDrumInteraction},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 38,
    vertical: 15,
};
const DIMENSIONS: Vector = Vector {
    horizontal: 2,
    vertical: 1,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OilDrum {
    linked_interactions: Vec<Interaction>,
}

impl OilDrum {
    pub fn initiate() -> Self {
        Self {
            linked_interactions: vec![Interaction::OilDrum(OilDrumInteraction::DipBranch(
                DipBranch,
            ))],
        }
    }
}

impl Item for OilDrum {
    fn get_name(&self) -> &'static str {
        "Oil Drum"
    }

    fn get_description(&self) -> &'static str {
        "Drum filled with oil"
    }

    fn get_detail_view(&self) -> String {
        let plain_drum = String::from(
            "
                                                          \r
                             _____________________        \r
                         ___/..................._.\\___   \r
                        /...................../ \\....\\  \r
                       |\\_....................\\ /...._/| \r
                       |..\\_________________________/..| \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...............................|  \r
                       |...........╔═════╗.............|  \r
                       |...........║.OIL.║.............|  \r
                       |...........╚═════╝.............|  \r
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

        plain_drum
            .chars()
            .fold(String::new(), |mut red_drum, c| {
                if c.is_whitespace() {
                    red_drum.push(c);
                } else {
                    red_drum.push_str(&format!(
                        "{}{}{}{}{}",
                        termion::color::Bg(termion::color::Red),
                        termion::color::Fg(termion::color::LightBlack),
                        c,
                        termion::color::Bg(termion::color::Reset),
                        termion::color::Fg(termion::color::Reset)
                    ))
                }
                red_drum
            })
            .replace(".", " ")
    }
}

impl Stationary for OilDrum {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let mut blocked_coordinates = Vec::new();
        for horizontal_offset in 0..DIMENSIONS.horizontal {
            for vertical_offset in 0..DIMENSIONS.vertical {
                blocked_coordinates.push(Vector {
                    horizontal: COORDINATES.horizontal + horizontal_offset,
                    vertical: COORDINATES.vertical + vertical_offset,
                });
            }
        }

        blocked_coordinates
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        InteractionStatus::Open
    }

    fn set_interaction_status_to_finished(&mut self) {}
}

impl Drawable for OilDrum {
    fn draw(&self) -> String {
        format!(
            "{}{}",
            termion::cursor::Goto(
                (COORDINATES.horizontal + 1) as u16,
                (COORDINATES.vertical + 1) as u16
            ),
            "🛢️"
        )
    }
}
