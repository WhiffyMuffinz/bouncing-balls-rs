mod assets;


fn main() {
    let m = assets::Vector{direction: 0.0, magnitude: 3.14};

    print!("{}",m.repr());
}
