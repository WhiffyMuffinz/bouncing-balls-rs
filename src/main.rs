mod assets;

use alea::{f32_less_than, f64_less_than, i32_less_than};
use assets::{Ball, Vector};
use raylib::prelude::*;

const WINDOW_DIMENSTIONS: [i32; 2] = [1280, 720];
const BG_COLOUR: Color = Color::new(0, 0, 0, 0);

#[allow(dead_code)]

fn collision(b: Ball, others: Vec<Ball>) -> () {
    //calculate the distance between the circles
    for other in others {
        let c1x = b.get_position_x();
        let c1y = b.get_position_y();
        let c2x = other.get_position_x();
        let c2y = other.get_position_y();
        let dist_x = c1x - c2x;
        let dist_y = c1y - c2y;
        //if colliding
        if (dist_x * dist_x + dist_y * dist_y) <= (b.get_mass() + other.get_mass()).powi(2) {
            //saving values to local variables
            let v1 = b.get_vector();
            let v2 = other.get_vector();
            let m1 = b.get_mass();
            let m2 = other.get_mass();
            let angle = (c1y - c2y).atan2(c1x - c2x);
            let tangent_vector = Vector {
                y: -(c2x - c1x),
                x: c2y - c1y,
            };
            let relative_velocity = Vector {
                x: (b.get_vector().get_x() - other.get_vector().get_x()),
                y: (b.get_vector().get_y() - other.get_vector().get_y()),
            };
            let length = relative_velocity.dot(&tangent_vector);
        }
    }
}

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
    };

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        b.update(WINDOW_DIMENSTIONS);
        d.clear_background(BG_COLOUR);

        b.render(&mut d);
    }
}
