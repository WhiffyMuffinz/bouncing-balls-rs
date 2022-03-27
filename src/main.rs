mod assets;

use alea::{f64_in_range, f64_less_than, i32_in_range};
use assets::{Ball, Vector};
use raylib::prelude::*;

const WINDOW_DIMENSTIONS: [i32; 2] = [500, 500];
const BG_COLOUR: Color = Color::new(0, 0, 0, 0);
const NUM_BALLS: u32 = 6;
const DEBUG: bool = false;
const MAX_BALL_SIZE: f64 = 20.0;

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

fn sweep_and_prune(balls: &Vec<Ball>) -> Vec<Vec<Ball>> {
    //returns vec of groups of possible collisions
    let mut sorted = balls.clone();
    let var: bool; //axis variable. true is x, false is y
    (sorted, var) = sort_by_axis(&sorted);
    let mut act_int: [f64; 2];
    let mut out: Vec<Vec<Ball>> = Vec::new();
    if var {
        //sweep and prune by x-axis
        act_int = [
            sorted[0].position_x - sorted[0].mass,
            sorted[0].position_x + sorted[0].mass,
        ];
        let mut added: Vec<Ball> = Vec::new();
        added.push(sorted[0].clone());
        for i in 1..sorted.len() {
            if sorted[i].position_x - sorted[i].mass <= act_int[1] {
                act_int[1] = sorted[i].position_x + sorted[i].mass;
                added.push(sorted[i].clone()); //what
            } else {
                out.push(added.clone());
                act_int = [
                    sorted[i].position_x - sorted[i].mass,
                    sorted[i].position_x + sorted[i].mass,
                ];
            }
        }
    } else {
        //sweep and prune by y-axis
        act_int = [
            sorted[0].position_y - sorted[0].mass,
            sorted[0].position_y + sorted[0].mass,
        ];
        let mut added: Vec<Ball> = Vec::new();
        added.push(sorted[0].clone());
        for i in 1..sorted.len() {
            if sorted[i].position_y - sorted[i].mass <= act_int[1] {
                act_int[1] = sorted[i].position_y + sorted[i].mass;
                added.push(sorted[i].clone());
            } else {
                out.push(added.clone());
                act_int = [
                    sorted[i].position_y - sorted[i].mass,
                    sorted[i].position_y + sorted[i].mass,
                ];
            }
        }
    }
    out
}

fn sort_by_axis(balls: &Vec<Ball>) -> (Vec<Ball>, bool) {
    //sorts by axis. returns sorted array, and boolean representing what axis it is sorted by
    let mut var_y: [f64; 2] = [0.0, 0.0];
    let mut var_x: [f64; 2] = [0.0, 0.0];
    let mut out = balls.clone();
    for _i in 0..3 {
        let ind = i32_in_range(0, (balls.len() - 1) as i32) as usize;
        if balls[ind].position_x >= var_x[1] {
            var_x[1] = balls[ind].position_x;
        } else if balls[ind].position_x <= var_x[0] {
            var_x[0] = balls[ind].position_x;
        }

        if balls[ind].position_y >= var_y[1] {
            var_y[1] = balls[ind].position_y;
        } else if balls[ind].position_y <= var_y[0] {
            var_y[0] = balls[ind].position_y;
        }
    }
    let var = var_y[1] - var_y[0] > var_x[1] - var_x[0];
    if var {
        quick_sort(&mut out, true);
    } else {
        quick_sort(&mut out, false)
    }
    (out, var)
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

fn collision(arr: Vec<Vec<Ball>>) {
    for mut coll in arr {
        if coll.len() < 1 {
            for i in 1..coll.len() {
                let b1 = coll[i - 1].clone();
                let b2 = coll[i].clone();
                //distance between balls squared because square root is slow
                let dist = (b1.position_x - b2.position_x).powi(2)
                    + (b1.position_y - b2.position_y).powi(2);
                //narrow phase collision check
                if dist <= (b1.mass + b2.mass).powi(2) {
                    let v1 = b1.get_velocity();
                    let v2 = b2.get_velocity();
                    let mut tangent_vector = Vector {
                        x: b2.position_x - b1.position_x,
                        y: -(b2.position_y - b1.position_y),
                    };
                    tangent_vector.normalize();
                    let relative_velocity = Vector {
                        x: v2.x - v1.x,
                        y: v2.y - v1.y,
                    };
                    let length = relative_velocity.dot(&tangent_vector);

                    let mut velocity_component_on_tangent = tangent_vector.clone();

                    velocity_component_on_tangent.multiply(length);

                    let velocity_component_perpendicular_to_tangent = Vector {
                        x: relative_velocity.x - velocity_component_on_tangent.x,
                        y: relative_velocity.y - velocity_component_on_tangent.y,
                    };

                    let mut out1 = Vector {
                        x: v1.x - velocity_component_perpendicular_to_tangent.x,
                        y: v1.y - velocity_component_perpendicular_to_tangent.y,
                    };
                    let mut out2 = Vector {
                        x: v2.x + velocity_component_perpendicular_to_tangent.x,
                        y: v2.y + velocity_component_perpendicular_to_tangent.y,
                    };

                    out1.normalize();
                    out2.normalize();
                    //apply the new computed vectors
                    coll[i - 1].vector = out1;
                    coll[i].vector = out2;
                }
            }
        }
    }
}

fn update(balls: &mut Vec<Ball>, balls2: &Vec<Ball>, dt: f32) {
    let mut possible_collisions: Vec<Vec<Ball>> = Vec::new();
    for ball in balls {
        ball.walk(dt);
        possible_collisions = sweep_and_prune(balls2);
        for c in &possible_collisions {
            println!("{:?}", c.len());
        }
        println!("\n");
        collision(possible_collisions);
        ball.handle_walls(WINDOW_DIMENSTIONS);
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
        update(&mut balls, &others, dt);
        others = balls.clone();
        d.clear_background(BG_COLOUR);
        for b in &balls {
            b.render(&mut d, DEBUG);
        }
    }
}
