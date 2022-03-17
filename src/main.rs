mod assets;

use alea::{f64_in_range, f64_less_than, i32_in_range};
use assets::{Ball, Vector};
use raylib::prelude::*;

const WINDOW_DIMENSTIONS: [i32; 2] = [1920, 1080];
const BG_COLOUR: Color = Color::new(0, 0, 0, 0);
const NUM_BALLS: u8 = 250;
const DEBUG: bool = false;
const MAX_BALL_SIZE: f64 = 20.0;

#[allow(dead_code)]

fn make_balls(num_balls: u8) -> Vec<Ball> {
    let mut out = vec![];
    for i in 0..num_balls {
        let b: Ball = Ball {
            colour: Color::new(
                i32_in_range(128, 256) as u8,
                i32_in_range(128, 256) as u8,
                i32_in_range(128, 256) as u8,
                i32_in_range(128, 256) as u8,
            ),
            mass: f64_in_range(5.0, MAX_BALL_SIZE),
            position_x: f64_less_than(WINDOW_DIMENSTIONS[0] as f64 - MAX_BALL_SIZE),
            position_y: f64_less_than(WINDOW_DIMENSTIONS[1] as f64 - MAX_BALL_SIZE),
            vector: Vector {
                x: (1.0 - f64_less_than(2.0)),
                y: (1.0 - f64_less_than(2.0)),
            },
            speed: 150.0,
            num: i,
        };
        out.push(b);
    }
    out
}

fn collision(balls: &mut Vec<Ball>) {
    for i in 0..balls.len() - 1 {
        let mut b = balls[i];
        let mut other = balls[i + 1];
        let c1x = b.get_position_x();
        let c1y = b.get_position_y();
        let c2x = other.get_position_x();
        let c2y = other.get_position_y();
        let v1 = b.get_vector();
        let v2 = other.get_vector();
        let m1 = b.get_mass();
        let m2 = other.get_mass();
        if other.num != b.num
            && (c2x - c1x) * (c2x - c1x) + (c2y - c1y) * (c2y - c1y) <= (m1 + m2) * (m1 + m2)
        {
            let mut tangent_vector = Vector {
                x: c2x - c1x,
                y: -(c2y - c1y),
            };
            tangent_vector.normalize();
            let relative_velocity = Vector {
                x: v2.x - v1.x,
                y: v2.y - v1.y,
            };
            let length = relative_velocity.dot(&tangent_vector);
            let mut vel_comp_on_tangent = tangent_vector.clone();
            vel_comp_on_tangent.multiply(length);
            let vel_comp_perp_tangent = Vector {
                x: relative_velocity.x - vel_comp_on_tangent.x,
                y: relative_velocity.y - vel_comp_on_tangent.y,
            };
            let mut out = Vector {
                x: v1.x - vel_comp_perp_tangent.x,
                y: v1.y - vel_comp_perp_tangent.y,
            };
            out.normalize();
            b.vector = out;
        }
    }
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
