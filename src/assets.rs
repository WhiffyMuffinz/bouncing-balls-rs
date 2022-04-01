use raylib::prelude::*;

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
    pub fn multiply_out(&self, scalar: &f64) -> Vector {
        Vector {
            x: scalar.clone() * self.get_x(),
            y: scalar.clone() * self.get_y(),
        }
    }
    pub fn clone(&self) -> Vector {
        Vector {
            x: self.get_x(),
            y: self.get_y(),
        }
    }
    pub fn add(&mut self, other: &Vector) {
        self.set_x(self.get_x() + other.get_x());
        self.set_y(self.get_y() + other.get_y());
    }
}

#[derive(Debug, Clone)]
pub struct Ball {
    pub colour: Color,
    pub mass: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub vector: Vector,
    pub speed: f64,
    pub num: u32,
}

impl Ball {
    #[allow(dead_code)]
    pub fn update(&mut self, window: [i32; 2], dt: f32, others: &Vec<Ball>, debug: bool) {
        self.position_x += self.vector.x * dt as f64 * self.speed;
        self.position_y += self.vector.y * dt as f64 * self.speed;
        self.collision(others, debug);
        self.handle_walls(window);
    }

    pub fn walk(&mut self, dt: f32) {
        self.position_x += self.vector.x * dt as f64 * self.speed;
        self.position_y += self.vector.y * dt as f64 * self.speed;
    }
    pub fn collision(&mut self, others: &Vec<Ball>, debug: bool) -> () {
        //something to speed up broad phase
        for other in others {
            let c1x = self.position_x;
            let c1y = self.position_y;
            let c2x = other.position_x;
            let c2y = other.position_y;
            let v1 = self.get_velocity();
            let v2 = other.get_velocity();
            let m1 = self.mass;
            let m2 = other.mass;
            if debug && self.num % 10 == 0 {
                self.write_to_file(&v1, &v2);
            }

            if other.num != self.num
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
                self.vector = out;
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

    pub fn get_velocity(&self) -> &Vector {
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
