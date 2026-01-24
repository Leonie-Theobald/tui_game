pub mod dropped_marshmallow_interaction;
pub mod fireplace_interaction;
pub mod firewood_interaction;
pub mod locked_door_interaction;
pub mod lost_chest_key_interaction;
pub mod oil_drum_interaction;
pub mod packet_interaction;
pub mod terminal_interaction;
pub mod torch_holder_interaction;
pub mod treasure_chest_interaction;

use crate::item::{
    collected_item::CollectedItems,
    interaction::{
        dropped_marshmallow_interaction::DroppedMarshmallowInteraction,
        fireplace_interaction::FireplaceInteraction, firewood_interaction::FirewoodInteraction,
        locked_door_interaction::LockedDoorInteraction,
        lost_chest_key_interaction::LostChestKeyInteraction,
        oil_drum_interaction::OilDrumInteraction, packet_interaction::PacketInteraction,
        terminal_interaction::TerminalInteraction,
        torch_holder_interaction::TorchHolderInteraction,
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
    Fireplace(FireplaceInteraction),
    DroppedMarshmallow(DroppedMarshmallowInteraction),
    OilDrum(OilDrumInteraction),
    TorchHolder(TorchHolderInteraction),
    Terminal(TerminalInteraction),
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
            Interaction::Fireplace(fireplace_interaction) => fireplace_interaction.get_inner(),
            Interaction::DroppedMarshmallow(dropped_marshmallow_interaction) => {
                dropped_marshmallow_interaction.get_inner()
            }
            Interaction::OilDrum(oil_drum_interaction) => oil_drum_interaction.get_inner(),
            Interaction::TorchHolder(torch_holder_interaction) => {
                torch_holder_interaction.get_inner()
            }
            Interaction::Terminal(terminal_interaction) => terminal_interaction.get_inner(),
        }
    }
}
