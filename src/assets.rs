

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
    red: f64,
    green: f64,
    blue: f64,

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

    pub fn Copy() {

    }
}

pub struct Ball {
    pub colour: Colour,
    pub mass: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub vector: Vector
}


impl Ball {
    pub fn repr (&self) -> String {
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

        out
    }

    pub fn get_position_x(&self) -> f64{
        return self.position_x;
    }

    pub fn get_position_y(&self) -> f64 {
        return self.position_y;
    }

    pub fn get_mass(&self) -> f64 {
        return self.mass;
    }

    //pub fn get_colour(&self) -> Colour {
    //    return self.colour;
    //}
}