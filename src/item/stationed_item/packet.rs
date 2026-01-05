use crate::{
    item::{
        Item,
        collected_item::CollectedItems,
        stationed_item::{
            Stationary,
            interaction::{Interaction, InteractionStatus, OpenPacket, ShakePacket, StompPacket},
        },
    },
    quest::{Quest, unwrap_packet::UnwrapPacket},
    tui::{
        Drawable, Vector,
        pop_up::{InfoPopUp, PopUp},
    },
};

const COORDINATES: Vector = Vector {
    horizontal: 15,
    vertical: 7,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    opened: bool,
    linked_quest: Quest,
    linked_interactions: Vec<Interaction>,
}

impl Packet {
    pub fn initiate() -> Self {
        let linked_quest = Quest::UnwrapPacket(UnwrapPacket);

        Self {
            opened: false,
            linked_quest,
            linked_interactions: vec![
                Interaction::OpenPacket(OpenPacket {}),
                Interaction::StompPacket(StompPacket {}),
                Interaction::ShakePacket(ShakePacket {}),
            ],
        }
    }
}

impl Item for Packet {
    fn get_name(&self) -> &'static str {
        "Wrapped Packet"
    }

    fn get_description(&self) -> &'static str {
        "A small packet wrapped in paper lying on the ground. Take a look inside!"
    }
}

impl Stationary for Packet {
    fn interact(&mut self, collected_items: &mut CollectedItems) {
        let prompts = self
            .linked_interactions
            .iter()
            .map(|interaction| interaction.get_inner().get_prompt())
            .collect();
        let pop_up = PopUp { options: prompts };
        let selected_interaction = match pop_up.get_user_selection() {
            Ok(selected_prompt) => self
                .linked_interactions
                .clone()
                .into_iter()
                .filter(|interaction| interaction.get_inner().get_prompt() == selected_prompt)
                .collect::<Vec<_>>()
                .first()
                .expect("Just had the user selection")
                .clone(),
            Err(_err) => return,
        };

        let info_prompt = selected_interaction
            .get_inner()
            .interact(self, collected_items);
        InfoPopUp {
            message: info_prompt,
        }
        .show_to_user();
    }

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

    fn get_interaction_status(&self) -> InteractionStatus {
        if self.opened {
            InteractionStatus::Finished
        } else {
            InteractionStatus::Open
        }
    }

    fn set_interaction_status_to_finished(&mut self) {
        self.opened = true;
    }
}

impl Drawable for Packet {
    fn draw(&self) -> String {
        let symbol = if self.opened { "X" } else { "📦" };

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
