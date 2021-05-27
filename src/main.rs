extern crate raylib;

use raylib::prelude::*;

mod game;
mod player;

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Wigman Game")
        .build();
    rl.set_target_fps(60);


// Draw ball


    while !rl.window_should_close() {
        //update_game(&mut game, &rl);
        //draw_game(&game, &mut rl, &thread);
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            dbg!("A");
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_circle(0, 0, 100.0, Color::GREEN);
    }
}
