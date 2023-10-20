//mod tests;
mod transform_map;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Object Transform")
        .size(800, 600)
        .build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
    }
}
