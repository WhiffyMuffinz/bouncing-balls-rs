

pub struct Vector {
    pub direction: f64,
    pub magnitude: f64,
}

impl Vector {

    pub fn repr(&self) -> String {
        let mut out = String::from("Vector with magnitude ");
        out += &self.magnitude.to_string();
        out += " and direction ";
        out += &self.direction.to_string();
        out
    }

    pub fn get_magnitude(&self) -> f64 {
        self.magnitude
    }

    pub fn get_direction(&self) -> f64 {
        self.direction
    }

    pub fn set_direction(&mut self, direction: f64) {
        self.direction = direction;
    }

    pub fn set_magnitude(&mut self, magnitude: f64) {
        self.magnitude = magnitude;
    }
}


pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,

}

impl Colour {
    
}