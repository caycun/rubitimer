use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn scramble() -> Vec<&'static str> {
    (1..15).map(|_| gen_str()).collect()
}

fn gen_str() -> &'static str {
    const ROTATIONS: &[&str] = &[
        "F", "D", "U", "L", "R", "F'", "D'", "U'", "L'", "R'", "F2", "D2", "U2", "L2", "R2",
    ];
    ROTATIONS.choose(&mut thread_rng()).unwrap()
}
