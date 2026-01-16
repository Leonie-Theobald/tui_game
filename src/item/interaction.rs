pub mod firewood_interaction;
pub mod locked_door_interaction;
pub mod lost_chest_key_interaction;
pub mod packet_interaction;
pub mod treasure_chest_interaction;

use crate::item::{
    collected_item::CollectedItems,
    interaction::{
        firewood_interaction::FirewoodInteraction, locked_door_interaction::LockedDoorInteraction,
        lost_chest_key_interaction::LostChestKeyInteraction, packet_interaction::PacketInteraction,
        treasure_chest_interaction::TreasureChestInteraction,
    },
    stationed_item::Stationary,
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
    LockedDoor(LockedDoorInteraction),
    LostChestKey(LostChestKeyInteraction),
    TreasureChest(TreasureChestInteraction),
    Firewood(FirewoodInteraction),
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
            Interaction::LockedDoor(door_interaction) => door_interaction.get_inner(),
            Interaction::LostChestKey(lost_chest_key_interaction) => {
                lost_chest_key_interaction.get_inner()
            }
            Interaction::TreasureChest(treasure_chest_interaction) => {
                treasure_chest_interaction.get_inner()
            }
            Interaction::Firewood(firewood_interaction) => firewood_interaction.get_inner(),
        }
    }
}
