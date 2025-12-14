mod game;
mod item;
mod player;

use game::Game;

fn main() {
    println!("INF: Game is initiated");
    let mut my_game = Game::initiate();

    println!(
        "{:?}, you can use these items:\n{:?}",
        my_game.get_player().show_name(),
        my_game.get_player().show_backpack()
    );
}
