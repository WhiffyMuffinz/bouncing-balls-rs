use raylib::prelude::*;

use libm::{atan2, cos, sin};
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}
impl Vector {
    pub fn repr(&self) -> String {
        let mut out = String::from("Vector with components X: ");
        out += &self.x.to_string();
        out += ", Y: ";
        out += &self.y.to_string();

        out
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.get_x() * other.get_x() + self.get_y() * other.get_y()
    }

    pub fn multiply(&mut self, scalar: f64) {
        self.set_x(self.get_x() * scalar);
        self.set_y(self.get_y() * scalar);
    }

    pub fn normalize(&mut self) {
        let magnitude = self.get_magnitude();
        self.set_x(self.get_x() / magnitude);
        self.set_y(self.get_y() / magnitude);
    }
    pub fn get_magnitude(&self) -> f64 {
        let x = self.get_x();
        let y = self.get_y();
        let mut magnitude = (x.powi(2) + y.powi(2)).sqrt();
        if magnitude.abs() < 1e-6 {
            magnitude = 1.0;
        }
        magnitude
    }
    pub fn clone(&self) -> Vector {
        Vector {
            x: self.get_x(),
            y: self.get_y(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    pub colour: Color,
    pub mass: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub vector: Vector,
    pub speed: f64,
    pub num: u8,
}

impl Ball {
    pub fn update(&mut self, window: [i32; 2], dt: f32, others: &Vec<Ball>, debug: bool) {
        self.position_x += self.vector.get_x() * dt as f64 * self.speed;
        self.position_y += self.vector.get_y() * dt as f64 * self.speed;
        self.collision(others, debug);
        self.handle_walls(window);
    }
    pub fn repr(&self) -> String {
        let mut out = String::from("Ball with mass: ");
        out += &self.mass.to_string();
        out += ", position: (";
        out += &self.position_x.to_string();
        out += ", ";
        out += &self.position_y.to_string();
        out += ", and ";
        out += &self.vector.repr();
        out += ")";

        out
    }

    pub fn collision(&mut self, others: &Vec<Ball>, debug: bool) -> () {
        for other in others {
            let c1x = self.get_position_x();
            let c1y = self.get_position_y();
            let c2x = other.get_position_x();
            let c2y = other.get_position_y();
            let v1 = self.get_vector();
            let v2 = other.get_vector();
            let m1 = self.get_mass();
            let m2 = other.get_mass();
            if debug && self.num % 10 == 0 {
                self.write_to_file(&v1, &v2);
            }

            if (c2x - c1x) * (c2x - c1x) + (c2y - c1y) * (c2y - c1y) <= (m1 + m2) * (m1 + m2) {
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
                self.vector = out;
            }
        }
    }
    fn reposition(&mut self, x: f64, y: f64, debug: bool) {
        if debug && self.num == 1 {
            println!(
                "Reposition from ({},{}) to ({}, {})",
                self.position_x, self.position_y, x, y
            );
        }
        self.position_x = x;
        self.position_y = y;
    }

    pub fn collision2(&mut self, others: &Vec<Ball>, debug: bool) -> () {
        for other in others {
            //calculate the distance between the circles
            if self.num != other.num {
                let c1x = self.position_x;
                let c1y = self.position_y;
                let c2x = other.get_position_x();
                let c2y = other.get_position_y();
                let v1 = self.vector;
                let v2 = other.get_vector();
                let m1 = self.mass;
                let m2 = other.get_mass();
                let dist_x = c1x - c2x;
                let dist_y = c1y - c2y;
                if !debug && self.num == 1 {
                    self.write_to_file(&v1, &v2);
                }
                //if colliding
                if (c2x - c1x) * (c2x - c1x) + (c2y - c1y) * (c2y - c1y)
                    <= (self.get_mass() + other.get_mass()) * (self.get_mass() + other.get_mass())
                {
                    //Issue seems to be because the particles are bouncing off each other's inner walls, and so they get stuck together.
                    //TODO find a way to unstick the particles
                    let angle = atan2(c2y - c1y, c2x - c1x);
                    let move_dist = m2 + m1 - (dist_x.powi(2) + dist_y.powi(2)).sqrt();

                    let out_x = c1x + cos(angle) * move_dist;
                    let out_y = c1y + sin(angle) * move_dist;

                    //self.reposition(out_x, out_y, debug);
                    let mut unit_normal = Vector {
                        x: c1x - c2x,
                        y: c1y - c2y,
                    };
                    unit_normal.normalize();
                    let unit_tangent = Vector {
                        x: -unit_normal.get_y(),
                        y: unit_normal.get_x(),
                    };

                    let v1n = unit_normal.dot(&v1);
                    let v2n = unit_normal.dot(v2);

                    let v_prime_1n = (v1n * (m1 - m2) + 2.0 * (m2 * v2n)) / (m1 + m2);

                    let mut out_norm = unit_normal.clone();
                    let mut out_tan = unit_tangent.clone();

                    out_tan.multiply(v_prime_1n);
                    out_norm.multiply(v_prime_1n);

                    let out = Vector {
                        x: out_norm.get_x() + out_tan.get_x(),
                        y: out_norm.get_y() + out_tan.get_y(),
                    };
                    self.vector = out;
                    self.vector.normalize();
                }
                self.vector = self.vector
            }
        }
    }
    pub fn render(&self, d: &mut RaylibDrawHandle, debug: bool) {
        d.draw_circle(
            self.position_x as i32,
            self.position_y as i32,
            self.mass as f32,
            self.colour,
        );
        if debug {
            d.draw_line(
                self.position_x as i32,
                self.position_y as i32,
                (self.position_x + self.vector.x * self.speed) as i32,
                (self.position_y + self.vector.y * self.speed) as i32,
                Color::GREEN,
            )
        }
    }
    fn write_to_file(&self, v1: &Vector, v2: &Vector) {
        let mut name: String = "log".to_owned();
        let n = (self.num as i32).to_string();
        name = name + &n;
        name = name + ".txt";
        //todo!("finish writing to log file");
        if !(Path::new(&name).exists()) {
            let mut f = File::create(&name).expect("unable to create file");
            println!("Created new file");
            write!(f, "Circle 1 {}. \t Circle 2  {}", v1.repr(), v2.repr())
                .expect("Access is Denied.");

            write!(f, "\n").expect("Access is Denied.");
        } else {
            let mut f = OpenOptions::new()
                .write(true)
                .read(true)
                .open(&name)
                .expect("Unable to Open File");
            let tmp = read_to_string(&name).expect("Access is Denied");
            write!(f, "{}", tmp).expect("Access is Denied");
            write!(f, "Circle 1 {}. \t Circle 2  {}", v1.repr(), v2.repr())
                .expect("Access is Denied.");

            write!(f, "\n").expect("Access is Denied.");
        }
    }

    pub fn get_position_x(&self) -> &f64 {
        return &self.position_x;
    }

    pub fn get_position_y(&self) -> &f64 {
        return &self.position_y;
    }

    pub fn get_mass(&self) -> &f64 {
        return &self.mass;
    }

    pub fn get_vector(&self) -> &Vector {
        &self.vector
    }

    pub fn handle_walls(&mut self, window: [i32; 2]) {
        //if the ball is intersecting and approaching the wall
        if self.position_x as f64 + self.mass >= window[0] as f64 && self.vector.x > 0.0 {
            self.vector.x *= -1.0;
        }
        if self.position_y as f64 + self.mass >= window[1] as f64 && self.vector.y > 0.0 {
            self.vector.y *= -1.0;
        }
        if self.position_x - self.mass <= 0.0 && self.vector.x < 0.0 {
            self.vector.x *= -1.0;
        }
        if self.position_y as f64 - self.mass <= 0.0 && self.vector.y < 0.0 {
            self.vector.y *= -1.0;
        }
    }
}
