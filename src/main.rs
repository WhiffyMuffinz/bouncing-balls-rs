mod assets;

fn main() {
    let m = assets::Ball {
        colour: assets::Colour {
            red: 0.7,
            green: 0.7,
            blue: 0.7,
        },
        mass: 1.0,
        position_x: 0.7,
        position_y: 0.7,
        vector: assets::Vector { x: 0.7, y: 0.6 },
    };

    print!("{}", m.repr());
}
