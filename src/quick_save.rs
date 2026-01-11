use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{
    item::{collected_item::CollectedItems, stationed_item::StationedItems},
    player::Player,
    quest::Quest,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QuickSave {
    player: Player,
    collected_items: CollectedItems,
    stationed_items: StationedItems,
    quests: Vec<Quest>,
}

impl QuickSave {
    pub fn from_current_game_state(
        player: Player,
        collected_items: CollectedItems,
        stationed_items: StationedItems,
        quests: Vec<Quest>,
    ) -> Self {
        Self {
            player,
            collected_items,
            stationed_items,
            quests,
        }
    }

    pub fn disassemble_to_game_state(self) -> (Player, CollectedItems, StationedItems, Vec<Quest>) {
        (
            self.player,
            self.collected_items,
            self.stationed_items,
            self.quests,
        )
    }

    fn get_path() -> String {
        let project_base =
            std::env::var("CARGO_MANIFEST_DIR").expect("Could not load CARGO_MANIFEST_DIR");
        format!("{project_base}/quick_saves/quick_save")
    }

    pub fn save_to_file(self) {
        let quick_save_serialized = serde_json::to_string(&self).expect("Could not serialize");

        let mut file = File::create(Self::get_path()).expect("Failed to create or open the file");
        file.write_all(quick_save_serialized.as_bytes())
            .expect("Failed to write to the file");
    }

    pub fn load_from_file() -> Self {
        let mut file = File::open(Self::get_path()).expect("Failed to open the file");
        let mut serialized_content = Vec::new();
        file.read_to_end(&mut serialized_content)
            .expect("Failed to read file");

        serde_json::from_str(
            &String::from_utf8(serialized_content).expect("Could not form String from utf8 slice"),
        )
        .expect("Could not deserialize")
    }
}
