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
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Interaction {
    OpenPacket(OpenPacket),
    StompPacket(StompPacket),
    ShakePacket(ShakePacket),
    BangDoor(BangDoor),
    BreakDownDoor(BreakDownDoor),
    UnlockWithKey(UnlockWithKey),
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
            Interaction::BangDoor(bang_door) => bang_door,
            Interaction::BreakDownDoor(break_down_door) => break_down_door,
            Interaction::UnlockWithKey(unlock_with_key) => unlock_with_key,
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

#[derive(Clone, Debug, PartialEq)]
pub struct StompPacket;
impl Interactable for StompPacket {
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

#[derive(Clone, Debug, PartialEq)]
pub struct ShakePacket;
impl Interactable for ShakePacket {
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

#[derive(Clone, Debug, PartialEq)]
pub struct BangDoor;
impl Interactable for BangDoor {
    fn get_prompt(&self) -> String {
        String::from("Loudly bang on the door")
    }

    fn interact(
        &self,
        _linked_stationary_item: &mut dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> String {
        String::from("Nobody opens")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BreakDownDoor;
impl Interactable for BreakDownDoor {
    fn get_prompt(&self) -> String {
        String::from("Break down door with raw force")
    }

    fn interact(
        &self,
        _linked_stationary_item: &mut dyn Stationary,
        _collected_items: &mut CollectedItems,
    ) -> String {
        String::from("This is a massive steel door. No chance")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnlockWithKey;
impl Interactable for UnlockWithKey {
    fn get_prompt(&self) -> String {
        String::from("Use key to open door")
    }

    fn interact(
        &self,
        linked_stationary_item: &mut dyn Stationary,
        collected_items: &mut CollectedItems,
    ) -> String {
        if !collected_items
            .list
            .contains(&CollectableItem::RustyKey(RustyKey))
        {
            return String::from("You have no key to open the door.");
        }

        // remove key
        let index = collected_items
            .list
            .iter()
            .position(|available_item| *available_item == CollectableItem::RustyKey(RustyKey))
            .expect("Checked before that item is available");
        collected_items.list.remove(index);

        // finish interaction status
        linked_stationary_item.set_interaction_status_to_finished();

        String::from("This was the correct key to use.")
    }
}
