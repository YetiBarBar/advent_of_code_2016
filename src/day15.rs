fn main() {
    /***********************
     My input
    Disc #1 has 13 positions; at time=0, it is at position 10.
    Disc #2 has 17 positions; at time=0, it is at position 15.
    Disc #3 has 19 positions; at time=0, it is at position 17.
    Disc #4 has 7 positions; at time=0, it is at position 1.
    Disc #5 has 5 positions; at time=0, it is at position 0.
    Disc #6 has 3 positions; at time=0, it is at position 1.
    ***********************/
    let points: Vec<_> = [(13, 10), (17, 15), (19, 17), (7, 1), (5, 0), (3, 1)]
        .iter()
        .zip(1..)
        .collect();

    // Ugly hardcoding, should have use CRT

    // We want (t + start_position + delta_t) % modulo == 0
    // so t % modulo == -delta_t - start_postion
    let res = (1..=i32::MAX).find(move |t| points.iter().all(|((a, b), c)| (b + t + c) % a == 0));
    println!("Part 1: {}", res.unwrap());

    let points: Vec<_> = [
        (13, 10),
        (17, 15),
        (19, 17),
        (7, 1),
        (5, 0),
        (3, 1),
        (11, 0),
    ]
    .iter()
    .zip(1..)
    .collect();

    // Ugly hardcoding, should use CRT
    let res = (1..=i32::MAX).find(move |t| points.iter().all(|((a, b), c)| (b + t + c) % a == 0));
    println!("Part 2: {}", res.unwrap());
}
