struct Range(u32, u32);

impl Range {
    fn is_rejected(&self, value: u32) -> bool {
        value >= self.0 && value <= self.1
    }
}
fn main() {
    let ranges: Vec<Range> = include_str!("../data/day_2016_20.data")
        // let ranges: Vec<Range> = r#"5-8
        // 0-2
        // 4-7"#
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(min, max)| Range(min.trim().parse().unwrap(), max.trim().parse().unwrap()))
        .collect();

    for val in 0..=u32::MAX {
        if ranges.iter().all(|range| !range.is_rejected(val)) {
            println!("Part 1: {val}");
            break;
        }
    }

    // Poorly long solution, took 37minutes !
    let valid_count = (0..=u32::MAX)
        .filter(|v| ranges.iter().any(|r| !r.is_rejected(*v)))
        .count();

    println!("Part 2: {valid_count}");
}
