use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{CollectableItem, CollectedItems, rusty_key::RustyKey},
    interaction::{Interactable, InteractionStatus},
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PacketInteraction {
    Open(Open),
    Stomp(Stomp),
    Shake(Shake),
}

impl PacketInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            PacketInteraction::Open(open_packet) => open_packet,
            PacketInteraction::Stomp(stomp_packet) => stomp_packet,
            PacketInteraction::Shake(shake_packet) => shake_packet,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Open;
impl Interactable for Open {
    fn get_prompt(&self) -> String {
        String::from("Open the packet")
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
                    .push(CollectableItem::RustyKey(RustyKey));
                linked_stationary_item.set_interaction_status_to_finished();
                String::from("Hurray! You found something.")
            }
            InteractionStatus::Finished => String::from("The packet has already been opened."),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stomp;
impl Interactable for Stomp {
    fn get_prompt(&self) -> String {
        String::from("Stomp on the packet")
    }

    fn interact(
        &self,
        _linked_stationary_item: &mut dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> String {
        String::from("Now the packet is flat. Well done...")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Shake;
impl Interactable for Shake {
    fn get_prompt(&self) -> String {
        String::from("Shake the packet")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> String {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => String::from("Something is rattling in the box."),
            InteractionStatus::Finished => String::from("Nothing to hear."),
        }
    }
}
