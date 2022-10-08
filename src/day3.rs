fn main() {
    let input = include_str!("../data/day3.data");

    let result = input
        .lines()
        .filter(|line| {
            let tuple = line_to_tuple(line);
            // println!("{:?} => {}", &tuple, is_triangle(tuple))
            is_triangle(tuple)
        })
        .count();

    println!("{}", result);

    let mut v0 = Vec::new();
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for line in input.lines() {
        let (a, b, c) = line_to_tuple(line);
        v0.push(a);
        v1.push(b);
        v2.push(c);
    }

    let v: Vec<_> = v0
        .iter()
        .chain(v1.iter())
        .chain(v2.iter())
        .copied()
        .collect();

    let res2 = v
        .chunks(3)
        .filter(|chunk| is_triangle((chunk[0], chunk[1], chunk[2])))
        .count();
    println!("{}", res2);
}

fn line_to_tuple(line: &str) -> (usize, usize, usize) {
    let v: Vec<_> = line.split_ascii_whitespace().filter(|t| t != &"").collect();
    (
        v[0].parse().unwrap(),
        v[1].parse().unwrap(),
        v[2].parse().unwrap(),
    )
}

fn is_triangle(a: (usize, usize, usize)) -> bool {
    a.0 + a.1 > a.2 && a.0 + a.2 > a.1 && a.1 + a.2 > a.0
}
