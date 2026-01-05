use crate::item::{
    collected_item::{CollectableItem, CollectedItems, rusty_key::RustyKey},
    stationed_item::Stationary,
};

pub enum InteractionStatus {
    Open,
    Finished,
}

pub trait Interactable {
    fn get_prompt(&self) -> String;

    fn interact(
        &self,
        linked_stationary_item: &dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> Result<String, String>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Interaction {
    OpenPacket(OpenPacket),
    StompPacket(StompPacket),
    ShakePacket(ShakePacket),
}

impl std::fmt::Display for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_inner().get_prompt())
    }
}

impl Interaction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            Interaction::OpenPacket(open_packet) => open_packet,
            Interaction::StompPacket(stomp_packet) => stomp_packet,
            Interaction::ShakePacket(shake_packet) => shake_packet,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenPacket;
impl Interactable for OpenPacket {
    fn get_prompt(&self) -> String {
        String::from("Open the packet")
    }

    fn interact(
        &self,
        linked_stationary_item: &dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> Result<String, String> {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => {
                collected_items
                    .list
                    .push(CollectableItem::RustyKey(RustyKey));
                Ok(String::from("Hurray! You found something."))
            }
            InteractionStatus::Finished => Ok(String::from("The packet has already been opened.")),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StompPacket;
impl Interactable for StompPacket {
    fn get_prompt(&self) -> String {
        String::from("Stomp on the packet")
    }

    fn interact(
        &self,
        _linked_stationary_item: &dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> Result<String, String> {
        Ok(String::from("Now the packet is flat. Well done..."))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShakePacket;
impl Interactable for ShakePacket {
    fn get_prompt(&self) -> String {
        String::from("Shake the packet")
    }

    fn interact(
        &self,
        linked_stationary_item: &dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> Result<String, String> {
        match linked_stationary_item.get_interaction_status() {
            InteractionStatus::Open => Ok(String::from("Something is rattling in the box.")),
            InteractionStatus::Finished => Ok(String::from("Nothing to hear.")),
        }
    }
}
