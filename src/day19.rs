fn josephus_turn(survivors: &[usize], start: bool) -> (Vec<usize>, bool) {
    let cycle = if start {
        [false, true].iter().cycle()
    } else {
        [true, false].iter().cycle()
    };

    (
        survivors
            .iter()
            .zip(cycle)
            .filter(|(_, c)| **c)
            .map(|(surv, _)| surv)
            .copied()
            .collect(),
        if survivors.len() % 2 == 0 {
            start
        } else {
            !start
        },
    )
}

fn main() {
    // let players: Vec<usize> = (1..=3_012_210).collect();
    // need to understand "Josephus problem"

    // Part 1:
    let mut start = false;
    let mut survivors: Vec<usize> = (1..=3_012_210).collect();
    while survivors.len() != 1 {
        (survivors, start) = josephus_turn(&survivors, start);
        // println!("{:?}", survivors);
    }
    println!("Part 1: {}", survivors.first().unwrap());
}
