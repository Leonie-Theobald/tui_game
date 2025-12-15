mod game;
mod item;
mod player;
mod quest;

use game::Game;

fn main() {
    println!("INF: Game is initiated");
    let mut my_game = Game::initiate();

    println!(
        "{}, you can use these items:\n{}",
        my_game.get_player().show_name(),
        my_game.get_player().show_backpack()
    );

    println!(
        "All your quests:\n{}",
        my_game.get_player().show_all_quests()
    );

    my_game.get_player().select_new_active_quest();

    my_game.get_player().solve_quest().unwrap();
    println!("The first quest is solved");

    println!(
        "Your quest list is empty: {}",
        my_game.get_player().show_unsolved_quests()
    );
    println!(
        "Your items are used: {}",
        my_game.get_player().show_backpack()
    );
}
