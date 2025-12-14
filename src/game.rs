use crate::{
    item::{Items, RustyKey},
    player::Player,
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
        };

        // add available item to players backpack
        let rusty_key = Items::RustyKey(RustyKey);
        player.backpack.push(rusty_key);

        Game { player }
    }

    pub fn get_player(&mut self) -> &mut Player {
        &mut self.player
    }
}
