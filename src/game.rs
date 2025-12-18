use crate::{
    item::{Items, RustyKey},
    player::Player,
    quest::{OpenRustyLock, QuestList, Quests},
    tui::{Orientation, Vector},
};

#[derive(Debug, Default)]
pub struct Game {
    player: Player,
}

impl Game {
    pub fn initiate() -> Self {
        let mut player = Player {
            name: "Leonie",
            backpack: Vec::new(),
            quests: QuestList::default(),
            coordinates: Vector {
                horizontal: 15,
                vertical: 14,
            },
            orientation: Orientation::North,
        };

        // add available item to players backpack
        let rusty_key = Items::RustyKey(RustyKey);
        player.backpack.push(rusty_key);

        // add first quest
        let open_rusty_lock = Quests::OpenRustyLock(OpenRustyLock::default());
        player.quests.list.push(open_rusty_lock);

        Game { player }
    }

    pub fn get_player(&mut self) -> &mut Player {
        &mut self.player
    }
}
