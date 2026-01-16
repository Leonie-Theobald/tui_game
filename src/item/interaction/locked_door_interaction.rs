use serde::{Deserialize, Serialize};

use crate::item::{
    collected_item::{CollectableItem, CollectedItems, rusty_key::RustyKey},
    interaction::Interactable,
    stationed_item::Stationary,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LockedDoorInteraction {
    Bang(Bang),
    BreakDown(BreakDown),
    UnlockWithKey(UnlockWithKey),
}

impl LockedDoorInteraction {
    pub fn get_inner(&self) -> &dyn Interactable {
        match self {
            LockedDoorInteraction::Bang(inner) => inner,
            LockedDoorInteraction::BreakDown(inner) => inner,
            LockedDoorInteraction::UnlockWithKey(inner) => inner,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bang;
impl Interactable for Bang {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BreakDown;
impl Interactable for BreakDown {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
