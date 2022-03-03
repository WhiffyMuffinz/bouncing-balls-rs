mod assets;

use alea::{f32_less_than, f64_less_than, i32_less_than};
use assets::{Ball, Vector};
use raylib::prelude::*;

const WINDOW_DIMENSTIONS: [i32; 2] = [1280, 720];
const BG_COLOUR: Color = Color::new(0, 0, 0, 0);

#[allow(dead_code)]

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_DIMENSTIONS[0], WINDOW_DIMENSTIONS[1])
        .title("Balls for Bakas")
        .build();
    let mut b: Ball = Ball {
        colour: Color::new(255, 255, 255, 255),
        mass: 10.0,
        position_x: (WINDOW_DIMENSTIONS[0] / 2) as f64,
        position_y: (WINDOW_DIMENSTIONS[1] / 2) as f64,
        vector: Vector { x: 1.0, y: 1.0 },
        speed: 100.0,
    };

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.draw_fps(10, 10);
        b.update(WINDOW_DIMENSTIONS, dt);
        d.clear_background(BG_COLOUR);

        b.render(&mut d);
    }
}
