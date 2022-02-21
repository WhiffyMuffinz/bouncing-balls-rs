pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn repr(&self) -> String {
        let mut out = String::from("Vector with components X: ");
        out += &self.x.to_string();
        out += "Y: ";
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

    pub fn dot(self, other: &Vector) -> f64 {
        self.get_x() * other.get_x() + self.get_y() * other.get_y()
    }

    pub fn multiply(mut self, scalar: f64) {
        //compute direction and magnitude of vector
        let theta = self.get_x().atan2(self.get_y());
        let mut magnitude = (self.get_x() + self.get_y()).sqrt();

        magnitude *= scalar;

        //using the direction and magnitude, compute the components and set them
        let tmp_x = magnitude * theta.cos();
        let tmp_y = magnitude * theta.sin();

        self.set_x(tmp_x);
        self.set_y(tmp_y);
    }

    pub fn multiply_multiple(mut self, scalars: Vec<f64>) {
        let theta = self.get_x().atan2(self.get_y());
        let mut magnitude = (self.get_x() + self.get_y()).sqrt();

        for scalar in scalars {
            magnitude *= scalar;
        }
        let tmp_x = magnitude * theta.cos();
        let tmp_y = magnitude * theta.sin();

        self.set_x(tmp_x);
        self.set_y(tmp_y);
    }
}

pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Colour {
    pub fn repr(&self) -> String {
        let mut out = String::from("Red: ");
        out += &self.red.to_string();
        out += " Green: ";
        out += &self.green.to_string();
        out += " Blue: ";
        out += &self.blue.to_string();
        out
    }
}

pub struct Ball {
    pub colour: Colour,
    pub mass: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub vector: Vector,
}

impl Ball {
    pub fn repr(&self) -> String {
        let mut out = String::from("Ball with colour: ");
        out += &self.colour.repr();
        out += ", mass: ";
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

    pub fn get_position_x(&self) -> &f64 {
        return &self.position_x;
    }

    pub fn get_position_y(&self) -> &f64 {
        return &self.position_y;
    }

    pub fn get_mass(&self) -> &f64 {
        return &self.mass;
    }

    pub fn get_colour(&self) -> &Colour {
        return &self.colour;
    }
    pub fn get_vector(&self) -> &Vector {
        &self.vector
    }

    pub fn handle_walls(mut self) {
        let x = self.vector.get_x();
        let y = self.vector.get_y();

        if self.position_x < 0.0 {
            self.vector.set_x(x * -1.0);
        }
        if self.position_x > 1.0 {
            self.vector.set_x(x * -1.0);
        }
        if self.position_y < 0.0 {
            self.vector.set_y(y * -1.0);
        }
        if self.position_y > 1.0 {
            self.vector.set_y(y * -1.0);
        }
    }
}
