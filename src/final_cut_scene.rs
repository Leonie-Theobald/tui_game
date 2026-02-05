mod cake;
mod leave_game;
mod room_with_cake;
mod sliding_wall_frames;
mod wall;

use crate::final_cut_scene::{
    cake::draw_cake,
    leave_game::prepare_leaving_game,
    wall::{show_plain_wall, slide_walls},
};

use std::time::Duration;

pub fn play_final_cut_scene() {
    show_plain_wall();
    std::thread::sleep(Duration::from_secs(1));
    slide_walls();
    std::thread::sleep(Duration::from_secs(1));
    draw_cake();
    prepare_leaving_game();
}
