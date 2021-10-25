mod assets;


fn main() {
    let m = assets::Vector{direction: 3.14, magnitude: 0.0};

    print!("{}",m.repr());
}
