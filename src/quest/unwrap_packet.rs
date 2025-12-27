use crate::{
    item::{
        collected_item::{CollectableItem, rusty_key::RustyKey},
        stationed_item::{StationaryItem, packet::Packet},
    },
    quest::IsQuest,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UnwrapPacket;

impl IsQuest for UnwrapPacket {
    fn get_description() -> &'static str {
        "The packet is lying on the ground. Take a look"
    }

    fn list_necessary_collected_items() -> Vec<CollectableItem> {
        Vec::new()
    }

    fn list_necessary_stationed_items() -> Vec<StationaryItem> {
        vec![StationaryItem::Packet(Packet::initiate())]
    }

    fn list_reward_collectable_items() -> Vec<CollectableItem> {
        vec![CollectableItem::RustyKey(RustyKey)]
    }

    fn list_reward_stationary_items() -> Vec<StationaryItem> {
        Vec::new()
    }
}
