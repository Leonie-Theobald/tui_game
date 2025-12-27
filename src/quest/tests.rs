use crate::{
    item::{
        collected_item::{CollectableItem, CollectedItems, candle::Candle, rusty_key::RustyKey},
        stationed_item::{
            StationaryItem, StationedItems, packet::Packet, treasure_chest::TreasureChest,
        },
    },
    quest::{IsQuest, QuestError},
};

struct TestQuest;
impl IsQuest for TestQuest {
    // placeholder implementation to test the default methods' implementations
    fn get_description() -> &'static str {
        todo!()
    }

    fn list_necessary_collected_items() -> Vec<CollectableItem> {
        vec![CollectableItem::RustyKey(RustyKey)]
    }

    fn list_necessary_stationed_items() -> Vec<StationaryItem> {
        vec![StationaryItem::TreasureChest(TreasureChest::initiate())]
    }

    fn list_reward_collectable_items() -> Vec<CollectableItem> {
        vec![CollectableItem::Candle(Candle)]
    }

    fn list_reward_stationary_items() -> Vec<StationaryItem> {
        vec![StationaryItem::Packet(Packet::initiate())]
    }
}

#[test]
fn try_solve_but_collectable_item_missing() {
    let collected_items = CollectedItems { list: Vec::new() };
    let stationed_items = StationedItems {
        list: vec![StationaryItem::TreasureChest(TreasureChest::initiate())],
    };

    assert_eq!(
        Err(QuestError::CollectableItemMissing),
        TestQuest::check_if_solvable(&collected_items, &stationed_items)
    );
}

#[test]
fn try_solve_but_stationed_item_missing() {
    let collected_items = CollectedItems {
        list: vec![CollectableItem::RustyKey(RustyKey)],
    };
    let stationed_items = StationedItems { list: Vec::new() };

    assert_eq!(
        Err(QuestError::StationaryItemMissing),
        TestQuest::check_if_solvable(&collected_items, &stationed_items)
    );
}

#[test]
fn solve_quest() {
    let collected_items = CollectedItems {
        list: vec![CollectableItem::RustyKey(RustyKey)],
    };
    let stationed_items = StationedItems {
        list: vec![StationaryItem::TreasureChest(TreasureChest::initiate())],
    };

    let reward_items = TestQuest::check_if_solvable(&collected_items, &stationed_items)
        .expect("Should be solvable");
    assert_eq!(
        reward_items.removed_stationed,
        vec![StationaryItem::TreasureChest(TreasureChest::initiate())]
    );
    assert_eq!(
        reward_items.removed_collectable,
        vec![CollectableItem::RustyKey(RustyKey)]
    );
    assert_eq!(
        reward_items.added_stationed,
        vec![StationaryItem::Packet(Packet::initiate())]
    );
    assert_eq!(
        reward_items.added_collectable,
        vec![CollectableItem::Candle(Candle)]
    );
}
