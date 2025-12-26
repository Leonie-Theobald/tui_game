use crate::item::{collected_item::CollectableItem, stationed_item::StationaryItem};

#[derive(Default)]
pub struct RewardItems {
    pub added_collectable: Vec<CollectableItem>,
    pub added_stationed: Vec<StationaryItem>,
    pub removed_collectable: Vec<CollectableItem>,
    pub removed_stationed: Vec<StationaryItem>,
}
