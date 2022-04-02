mod assets;

use alea::{f64_in_range, f64_less_than, i32_in_range};
use assets::{Ball, Vector};
use raylib::prelude::*;

const WINDOW_DIMENSTIONS: [i32; 2] = [500, 500];
const BG_COLOUR: Color = Color::new(0, 0, 0, 0);
const NUM_BALLS: u32 = 6;
const DEBUG: bool = false;
const MAX_BALL_SIZE: f64 = 50.0;

fn make_balls(num_balls: u32) -> Vec<Ball> {
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
fn make_balls_2() -> Vec<Ball> {
    let b1 = Ball {
        colour: Color::new(255, 0, 0, 255),
        mass: 50.0,
        position_x: 300.0,
        position_y: 250.0,
        vector: Vector { x: 0.0, y: 0.1 },
        speed: 150.0,
        num: 0,
    };
    let b2 = Ball {
        colour: Color::new(0, 255, 0, 255),
        mass: 50.0,
        position_x: 200.0,
        position_y: 250.0,
        vector: Vector { x: 1.0, y: 0.0 },
        speed: 150.0,
        num: 1,
    };
    let b3 = Ball {
        colour: Color::new(0, 0, 255, 255),
        mass: 50.0,
        position_x: 100.0,
        position_y: 250.0,
        vector: Vector { x: 0.0, y: 1.0 },
        speed: 150.0,
        num: 2,
    };
    vec![b1, b2, b3]
}

fn quick_sort(arr: &mut Vec<Ball>, var: bool) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize, var);
}

fn _quick_sort(arr: &mut Vec<Ball>, low: isize, high: isize, var: bool) {
    if low < high {
        let p;
        if var {
            p = partition_x(arr, low, high);
        } else {
            p = partition_y(arr, low, high);
        }
        _quick_sort(arr, low, p - 1, var);
        _quick_sort(arr, p + 1, high, var);
    }
}

fn partition_x(arr: &mut Vec<Ball>, low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize].position_x < arr[pivot].position_x {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize].position_x > arr[pivot].position_x {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}

fn partition_y(arr: &mut Vec<Ball>, low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize].position_y < arr[pivot].position_y {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize].position_y > arr[pivot].position_y {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}

fn sort_by_axis(balls: &mut Vec<Ball>) -> bool {
    let mut x_variation: [f64; 2] = [f64::MAX, 0.0];
    let mut y_variation: [f64; 2] = [f64::MAX, 0.0];

    for _i in 0..3 {
        let ind = i32_in_range(0, balls.len() as i32 - 1) as usize;
        if balls[ind].position_x < x_variation[0] {
            x_variation[0] = balls[ind].position_x;
        };
        if balls[ind].position_x > x_variation[1] {
            x_variation[1] = balls[ind].position_x;
        }
        if balls[ind].position_y < y_variation[0] {
            y_variation[0] = balls[ind].position_y;
        };
        if balls[ind].position_y > y_variation[1] {
            y_variation[1] = balls[ind].position_y;
        }
    }

    let var = x_variation[1] - x_variation[0] > y_variation[1] - y_variation[0];
    quick_sort(balls, var);
    var
}
fn sweep_and_prune(balls: &mut Vec<Ball>) -> Vec<Vec<usize>> {
    let var = sort_by_axis(balls);
    let mut act_int: [f64; 2];
    //outputs a vec that contains a group of the indeces of possible collisions between balls
    // meanig that index 0 of the main vec will contain a vec with some number of indeces that will reference the indeces of the main ball array
    let mut out: Vec<Vec<usize>> = Vec::new();
    let mut added: Vec<usize> = Vec::new();
    if var {
        act_int = [
            balls[0].position_x - balls[0].mass,
            balls[0].position_x + balls[0].mass,
        ];
        let mut i = 0;
        while i < balls.len() {
            let b = &balls[i];
            //see if the particle intersects with the active interval
            if b.position_x - b.mass <= act_int[1] {
                //update the interval
                act_int[1] = b.position_x + b.mass;
                //push particle to collision
                added.push(i);
                if added.len() == 2 {
                    out.push(added);
                    added = Vec::new();
                }
            } else {
                //update active interval
                out.push(added.clone());
                //print!("{:?}", act_int);
                act_int[0] = b.position_x - b.mass;
                act_int[1] = b.position_x + b.mass;
                //println!("{:?}", act_int);
                //println!("{:?}", added.len());
                //reset collision vec
                added.clear();
            }
            if out.len() == 0 {
                out.push(added.clone());
            }
            i += 1;
        }
    } else {
        act_int = [
            balls[0].position_y - balls[0].mass,
            balls[0].position_y + balls[0].mass,
        ];
        for i in 0..balls.len() {
            let b = &balls[i];
            //see if the particle intersects with the active interval
            if b.position_y - b.mass <= act_int[1] {
                //update the interval
                act_int[1] = b.position_y + b.mass;
                //push particle to collision
                added.push(i);
            } else {
                //update active interval
                out.push(added.clone());
                //print!("{:?}", act_int);
                act_int[0] = b.position_y - b.mass;
                act_int[1] = b.position_y + b.mass;
                //println!("{:?}", act_int);
                //println!("{:?}", added.len());
                //reset collision vec
                added.clear();
            }
            if out.len() == 0 {
                out.push(added.clone());
            }
        }
    }
    println!("{:?}", out.len());
    out
}

fn handle_balls(balls: &mut Vec<Ball>, balls2: &Vec<Ball>) {
    let collisions = sweep_and_prune(balls);
    //let collisions: Vec<Vec<usize>> = vec![vec![0, 1]];
    //loop over collisions and extract the balls that are colliding from balls2[]
    //then calculate the new vectors for those balls
    //then apply those vectors to the balls

    for collision in collisions {
        if collision.len() > 1 {
            for i in 0..collision.len() - 1 {
                let velocity_1 = balls2[collision[i + 1]].vector;
                let velocity_2 = balls2[collision[i]].vector;
                let mass_1 = balls2[collision[i + 1]].mass;
                let mass_2 = balls2[collision[i]].mass;
                let position_1 = [
                    balls2[collision[i + 1]].position_x,
                    balls2[collision[i + 1]].position_y,
                ];
                let position_2 = [
                    balls2[collision[i]].position_x,
                    balls2[collision[i]].position_y,
                ];
                //if the distance between the balls is less than the sum of their radii and the space between them isn't increasing
                if velocity_1.dot(&velocity_2) <= 0.0
                    && ((position_1[0] - position_2[0]).powi(2)
                        + (position_1[1] - position_2[1]).powi(2))
                    .sqrt()
                        < (mass_1 + mass_2)
                {
                    print!("collision");
                    //calculate the new vectors
                    let mut unit_normal = Vector {
                        x: position_1[0] - position_2[0],
                        y: position_1[1] - position_2[1],
                    };
                    unit_normal.normalize();
                    let unit_tangent = Vector {
                        x: -unit_normal.y,
                        y: unit_normal.x,
                    };
                    let velocity_1_tangent = velocity_1.dot(&unit_tangent);
                    let velocity_2_tangent = velocity_2.dot(&unit_tangent);
                    let velocity_1_normal = velocity_1.dot(&unit_normal);
                    let velocity_2_normal = velocity_2.dot(&unit_normal);
                    let v_prime_1_tangent = velocity_1_tangent;
                    let v_prime_2_tangent = velocity_2_tangent;

                    let v_prime_1_normal = (velocity_1_normal * (mass_1 - mass_2)
                        + 2.0 * mass_2 * velocity_2_normal)
                        / (mass_1 + mass_2);
                    let v_prime_2_normal = (velocity_2_normal * (mass_2 - mass_1)
                        + 2.0 * mass_1 * velocity_1_normal)
                        / (mass_1 + mass_2);
                    let mut out_norm_1 = unit_normal.multiply_out(&v_prime_1_normal);
                    let mut out_norm_2 = unit_normal.multiply_out(&v_prime_2_normal);
                    let out_tan_1 = unit_tangent.multiply_out(&v_prime_1_tangent);
                    let out_tan_2 = unit_tangent.multiply_out(&v_prime_2_tangent);

                    out_norm_1.add(&out_tan_1);
                    out_norm_2.add(&out_tan_2);
                    balls[collision[i + 1]].vector = out_norm_1;
                    balls[collision[i]].vector = out_norm_2;
                }
            }
        }
    }
}

fn update(balls: &mut Vec<Ball>, balls2: &Vec<Ball>, dt: f32) {
    handle_balls(balls, balls2);
    for ball in balls {
        ball.walk(dt);
        ball.handle_walls(WINDOW_DIMENSTIONS);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_DIMENSTIONS[0], WINDOW_DIMENSTIONS[1])
        .title("Balls for Bakas")
        .build();
    //let mut balls = make_balls(NUM_BALLS);
    let mut balls = make_balls_2();
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.draw_fps(10, 10);
        let balls2 = balls.clone();
        update(&mut balls, &balls2, dt);
        //for ball in &mut balls {
        //    ball.update(WINDOW_DIMENSTIONS, dt, &balls2, DEBUG);
        //}
        d.clear_background(BG_COLOUR);
        for b in &balls {
            b.render(&mut d, DEBUG);
        }
    }
}
