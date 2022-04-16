mod assets;

use alea::{f64_in_range, f64_less_than, i32_in_range};
use assets::{Ball, Vector};
use libm::{atan2, cos, sin};
use raylib::prelude::*;

use rand::distributions::{Distribution, Uniform};

const WINDOW_DIMENSTIONS: [i32; 2] = [1280, 720];
const BG_COLOUR: Color = Color::new(0, 0, 0, 0);
const NUM_BALLS: u32 = 50;
const DEBUG: bool = false;
const MAX_BALL_SIZE: f64 = 25.0;

fn make_balls(num_balls: u32) -> Vec<Ball> {
    let mut rng = rand::thread_rng();
    let range_x = Uniform::from(0.0..WINDOW_DIMENSTIONS[0] as f64 - MAX_BALL_SIZE);
    let range_y = Uniform::from(0.0..WINDOW_DIMENSTIONS[1] as f64 - MAX_BALL_SIZE);
    let mut out = vec![];
    for i in 0..num_balls {
        let b: Ball = Ball {
            colour: Color::new(
                i32_in_range(128, 256) as u8,
                i32_in_range(128, 256) as u8,
                i32_in_range(128, 256) as u8,
                i32_in_range(128, 256) as u8,
            ),
            mass: f64_in_range(15.0, MAX_BALL_SIZE),
            position_x: range_x.sample(&mut rng),
            position_y: range_y.sample(&mut rng),
            vector: Vector {
                x: (1.0 - f64_less_than(2.0)),
                y: (1.0 - f64_less_than(2.0)),
            },
            speed: 150.0,
            num: i,
        };
        out.push(b);
    }
    //to try to ensure the balls aren't overlapping

    for i in 0..out.len() {
        for j in 0..out.len() {
            if i != j {
                let mut dist = (out[i].position_x - out[j].position_x).powi(2)
                    + (out[i].position_y - out[j].position_y).powi(2);
                dist = dist.sqrt();
                if dist < out[i].mass + out[j].mass {
                    let angle = atan2(
                        out[j].position_y - out[i].position_y,
                        out[j].position_x - out[i].position_x,
                    );
                    let dist_to_move = out[i].mass + out[j].mass - dist;
                    out[i].position_x += dist_to_move * cos(angle);
                    out[i].position_y += dist_to_move * sin(angle);
                }
            }
        }
    }
    out
}

fn make_balls_2() -> Vec<Ball> {
    let b1 = Ball {
        colour: Color::new(255, 0, 0, 255),
        mass: 50.0,
        position_x: 250.0,
        position_y: 50.0,
        vector: Vector { x: 1.0, y: 1.0 },
        speed: 150.0,
        num: 0,
    };
    let b2 = Ball {
        colour: Color::new(0, 255, 255, 255),
        mass: 50.0,
        position_x: 50.0,
        position_y: 150.0,
        vector: Vector { x: 1.0, y: 1.0 },
        speed: 150.0,
        num: 1,
    };
    let b3 = Ball {
        colour: Color::new(0, 0, 255, 255),
        mass: 50.0,
        position_x: 50.0,
        position_y: 250.0,
        vector: Vector { x: 1.0, y: 1.0 },
        speed: 150.0,
        num: 2,
    };
    let b4 = Ball {
        colour: Color::new(255, 255, 0, 255),
        mass: 50.0,
        position_x: 50.0,
        position_y: 350.0,
        vector: Vector { x: 1.0, y: 1.0 },
        speed: 150.0,
        num: 3,
    };
    vec![b1, b4, b3, b2]
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

    let var = true;
    // let var = x_variation[1] - x_variation[0] > y_variation[1] - y_variation[0];
    quick_sort(balls, var);
    var
}

fn sweep_and_prune(balls: &mut Vec<Ball>) -> (Vec<Vec<usize>>, bool) {
    let var = sort_by_axis(balls);
    let mut act_int: [f64; 2];
    //outputs a vec that contains a group of the indeces of possible collisions between balls
    let mut out: Vec<Vec<usize>> = Vec::new();
    let mut added: Vec<usize> = Vec::new();
    if var {
        act_int = [
            balls[0].position_x - balls[0].mass,
            balls[0].position_x + balls[0].mass,
        ];
        for i in 0..balls.len() {
            let b = &balls[i];
            let ball_interval = [b.position_x - b.mass, b.position_x + b.mass];
            if ball_interval[0] <= act_int[1] {
                added.push(i);
                act_int[1] = ball_interval[1];
            } else {
                out.push(added.clone());
                added = Vec::new();
                act_int = [ball_interval[0], ball_interval[1]];
            }
        }
        out.push(added.clone());
        // added = Vec::new();
    } else {
        act_int = [
            balls[0].position_y - balls[0].mass,
            balls[0].position_y + balls[0].mass,
        ];
        let mut i = 0;
        while i < balls.len() {
            let b = &balls[i];
            if b.position_y - b.mass <= act_int[1] {
                added.push(i);
                act_int[1] = b.position_y + b.mass;
            } else {
                out.push(added.clone());
                added = Vec::new();
                act_int[0] = b.position_y - b.mass;
                act_int[1] = b.position_y + b.mass;
            }
            i += 1;
        }
        if out.len() == 0 {
            out.push(added.clone());
            added = Vec::new();
        }
    }
    (out, var)
}

fn handle_balls(balls: &mut Vec<Ball>) -> bool {
    let (collisions, var) = sweep_and_prune(balls);
    let balls2 = balls.clone();
    let mut collided: Vec<[u32; 2]> = vec![]; //debug variable

    //loop over collisions and extract the balls that are colliding from balls2[]
    //then calculate the new vectors for those balls
    //then apply those vectors to the balls

    for collision in collisions {
        for i in 0..collision.len() {
            for j in 0..collision.len() {
                if i != j {
                    let velocity_1 = balls2[collision[i]].vector;
                    let velocity_2 = balls2[collision[j]].vector;
                    let mass_1 = balls2[collision[i]].mass;
                    let mass_2 = balls2[collision[j]].mass;
                    let position_1 = [
                        balls2[collision[i]].position_x,
                        balls2[collision[i]].position_y,
                    ];
                    let position_2 = [
                        balls2[collision[j]].position_x,
                        balls2[collision[j]].position_y,
                    ];
                    //if the distance between the balls is less than the sum of their radii and the space between them isn't increasing
                    // velocity_1.dot(&velocity_2) <= 0.0 &&
                    if ((position_1[0] - position_2[0]).powi(2)
                        + (position_1[1] - position_2[1]).powi(2))
                        <= (mass_1 + mass_2).powi(2)
                    {
                        if var {
                            print!("x ");
                        } else {
                            print!("y ");
                        }
                        println!(
                            "collision between nums {}, {} at indeces {},{}, collision length {}",
                            balls2[collision[i]].num,
                            balls2[collision[j]].num,
                            collision[i],
                            collision[j],
                            collision.len()
                        );
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
                        out_norm_1.normalize();
                        out_norm_2.normalize();
                        balls[collision[i]].vector = out_norm_1;
                        balls[collision[j]].vector = out_norm_2;
                        if DEBUG {
                            collided.push([balls[collision[i]].num, balls[collision[j]].num]);
                        }
                    }
                }
            }
        }
    }

    if DEBUG {
        for i in 0..balls2.len() {
            for j in 0..balls2.len() {
                if i != j {
                    let ball = &balls2[i];
                    let other = &balls2[j];
                    //see if they're actually colliding
                    if (ball.position_x - other.position_x).powi(2)
                        + (ball.position_y - other.position_y).powi(2)
                        <= (ball.mass + other.mass).powi(2)
                    {
                        //see if the collision was detected in the previous loop
                        if !collided.contains(&[ball.num, other.num]) {
                            println!(
                                "collision between nums {}, {} not detected at indeces {},{}, ",
                                ball.num, other.num, i, j
                            );
                        }
                    }
                }
            }
        }
    }
    var
}

fn handle_balls_2(balls: &mut Vec<Ball>) -> bool {
    let balls2 = balls.clone();

    for i in 0..balls.len() {
        for j in 1..balls.len() {
            if i != j {
                let velocity_1 = balls2[j].vector;
                let velocity_2 = balls2[i].vector;
                let mass_1 = balls2[j].mass;
                let mass_2 = balls2[i].mass;
                let position_1 = [balls2[j].position_x, balls2[j].position_y];
                let position_2 = [balls2[i].position_x, balls2[i].position_y];
                //if the distance between the balls is less than the sum of their radii and the space between them isn't increasing
                //
                // && velocity_1.dot(&velocity_2) != -1.0 &&

                if velocity_1.dot(&velocity_2) != 1.0
                    && ((position_1[0] - position_2[0]).powi(2)
                        + (position_1[1] - position_2[1]).powi(2))
                        <= (mass_1 + mass_2).powi(2)
                {
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
                    balls[j].vector = out_norm_1;
                    balls[i].vector = out_norm_2;
                }
            }
        }
    }
    true
}

fn update(balls: &mut Vec<Ball>, dt: f32) -> bool {
    // let var = handle_balls_2(balls);
    let var = handle_balls_2(balls);
    for ball in balls {
        ball.walk(dt);
        ball.handle_walls(WINDOW_DIMENSTIONS);
    }
    var
}

fn main() {
    let mut balls = make_balls(NUM_BALLS);
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_DIMENSTIONS[0], WINDOW_DIMENSTIONS[1])
        .title("Balls for Bakas")
        .build();
    // let mut balls = make_balls_2();
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);

        d.draw_fps(10, 10);
        let var = update(&mut balls, dt);

        d.clear_background(BG_COLOUR);
        for b in &balls {
            b.render(&mut d, DEBUG, var);
        }
    }
}
