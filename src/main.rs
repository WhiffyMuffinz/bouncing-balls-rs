mod assets;

use alea::{f64_in_range, f64_less_than, i32_in_range, i32_less_than};
use assets::{Ball, Vector};
use raylib::prelude::*;

const WINDOW_DIMENSTIONS: [i32; 2] = [1280, 720];
const BG_COLOUR: Color = Color::new(0, 0, 0, 0);
const NUM_BALLS: u8 = 50;
const DEBUG: bool = true;
const MAX_BALL_SIZE: f64 = 20.0;

#[allow(dead_code)]

fn make_balls(num_balls: u8) -> Vec<Ball> {
    let mut out = vec![];
    for i in 0..num_balls {
        let mut b: Ball = Ball {
            colour: Color::new(
                i32_less_than(256) as u8,
                i32_less_than(256) as u8,
                i32_less_than(256) as u8,
                i32_in_range(128, 256) as u8,
            ),
            mass: f64_in_range(5.0, MAX_BALL_SIZE),
            position_x: f64_less_than(WINDOW_DIMENSTIONS[0] as f64 - MAX_BALL_SIZE),
            position_y: f64_less_than(WINDOW_DIMENSTIONS[1] as f64 - MAX_BALL_SIZE),
            vector: Vector {
                x: (1.0 - f64_less_than(2.0)) / 1.0,
                y: (1.0 - f64_less_than(2.0)) / 1.0,
            },
            speed: 100.0,
            num: i,
        };
        out.push(b);
    }
    out
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_DIMENSTIONS[0], WINDOW_DIMENSTIONS[1])
        .title("Balls for Bakas")
        .build();
    let mut balls = make_balls(NUM_BALLS);

    let mut others = balls.clone();
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.draw_fps(10, 10);
        for b in &mut balls {
            b.update(WINDOW_DIMENSTIONS, dt, &others, DEBUG);
        }
        others = balls.clone();
        d.clear_background(BG_COLOUR);
        for b in &balls {
            b.render(&mut d, DEBUG);
        }
    }
}
