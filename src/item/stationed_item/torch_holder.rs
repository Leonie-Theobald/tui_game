use serde::{Deserialize, Serialize};

use crate::{
    item::{
        Item,
        interaction::{
            Interaction, InteractionStatus,
            torch_holder_interaction::{MountTorch, TorchHolderInteraction},
        },
        stationed_item::Stationary,
    },
    tui::{Drawable, Vector},
};

const COORDINATES: Vector = Vector {
    horizontal: 30,
    vertical: 4,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TorchHolder {
    linked_interactions: Vec<Interaction>,
    holds_torch: bool,
}

impl TorchHolder {
    pub fn initiate() -> Self {
        Self {
            linked_interactions: vec![Interaction::TorchHolder(
                TorchHolderInteraction::MountTorch(MountTorch),
            )],
            holds_torch: false,
        }
    }

    fn get_room_setup(&self) -> String {
        let room_setup = "..............
...  .........
.     o.......
      ┸";

        if self.holds_torch {
            // remove dark areas and place fire into torch holder
            room_setup.replace('.', " ").replace('o', "🔥")
        } else {
            room_setup.to_owned()
        }
    }
}

impl Item for TorchHolder {
    fn get_name(&self) -> &'static str {
        "Torch Holder"
    }

    fn get_description(&self) -> &'static str {
        "This can hold one torch at the wall."
    }

    fn get_detail_view(&self) -> String {
        todo!()
    }
}

impl Stationary for TorchHolder {
    fn list_blocking_coordinates(&self) -> Vec<Vector> {
        let room_setup = self.get_room_setup();

        let mut blocked_coordinates = Vec::new();
        for (vertical_offset, line) in room_setup.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    'o' | '┸' => blocked_coordinates.push(Vector {
                        horizontal: COORDINATES.horizontal + horizontal_offset,
                        vertical: COORDINATES.vertical + vertical_offset,
                    }),
                    '🔥' => {
                        // fire needs to horizontal blockages
                        blocked_coordinates.push(Vector {
                            horizontal: COORDINATES.horizontal + horizontal_offset,
                            vertical: COORDINATES.vertical + vertical_offset,
                        });
                        blocked_coordinates.push(Vector {
                            horizontal: COORDINATES.horizontal + horizontal_offset + 1,
                            vertical: COORDINATES.vertical + vertical_offset,
                        });
                    }
                    _ => continue,
                }
            }
        }

        blocked_coordinates
    }

    fn get_linked_interactions(&self) -> &Vec<Interaction> {
        &self.linked_interactions
    }

    fn get_interaction_status(&self) -> InteractionStatus {
        if self.holds_torch {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.holds_torch = true;
    }
}

impl Drawable for TorchHolder {
    fn draw(&self) -> String {
        let room_setup = self.get_room_setup();

        let mut room = String::new();
        for (vertical_offset, line) in room_setup.lines().enumerate() {
            for (horizontal_offset, char) in line.chars().enumerate() {
                match char {
                    ' ' => continue,
                    '.' => room.push_str(&format!(
                        "{}{} {}",
                        termion::cursor::Goto(
                            (COORDINATES.horizontal + horizontal_offset + 1) as u16,
                            (COORDINATES.vertical + vertical_offset + 1) as u16
                        ),
                        termion::color::Bg(termion::color::LightBlack),
                        termion::color::Bg(termion::color::Reset),
                    )),
                    _ => room.push_str(&format!(
                        "{}{}",
                        termion::cursor::Goto(
                            (COORDINATES.horizontal + horizontal_offset + 1) as u16,
                            (COORDINATES.vertical + vertical_offset + 1) as u16
                        ),
                        char,
                    )),
                }
            }
        }

        room
    }
}
