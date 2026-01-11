use crate::item::{
    collected_item::CollectedItems,
    stationed_item::{
        Stationary, blocked_room::DoorInteraction, lost_chest_key::LostChestKeyInteraction,
        packet::PacketInteraction, treasure_chest::TreasureChestInteraction,
    },
};
use serde::{Deserialize, Serialize};

pub enum InteractionStatus {
    Open,
    Finished,
}

pub trait Interactable {
    fn get_prompt(&self) -> String;

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Interaction {
    Packet(PacketInteraction),
    Door(DoorInteraction),
    LostChestKey(LostChestKeyInteraction),
    TreasureChest(TreasureChestInteraction),
}

impl std::fmt::Display for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_inner().get_prompt())
    }
}

impl Interaction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            Interaction::Packet(packet_interaction) => packet_interaction.get_inner(),
            Interaction::Door(door_interaction) => door_interaction.get_inner(),
            Interaction::LostChestKey(lost_chest_key_interaction) => {
                lost_chest_key_interaction.get_inner()
            }
            Interaction::TreasureChest(treasure_chest_interaction) => {
                treasure_chest_interaction.get_inner()
            }
        }
    }
}
