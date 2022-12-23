use rand::{thread_rng, Rng};

pub fn scramble() -> Vec<&'static str> {
    let rotations = vec![
        "F", "D", "U", "L", "R", "F'", "D'", "U'", "L'", "R'", "F2", "D2", "U2", "L2", "R2",
    ];
    let mut moves = vec![];
    for _i in 1..15 {
        moves.push(rotations[gen_number() as usize]);
    }
    moves
}

fn gen_number() -> u32 {
    let mut rng = thread_rng();
    let x: u32 = rng.gen_range(0..14);
    return x;
}
