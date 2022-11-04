struct Range(u32, u32);

impl Range {
    fn is_accepted(&self, value: u32) -> bool {
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

    // for val in 0..=u32::MAX {
    //     if ranges.iter().all(|range| range.is_accepted(val)) {
    //         println!("Lowest accpted: {}", val);
    //         break;
    //     }
    // }

    let (min, max) = ranges
        .iter()
        .fold((u32::MAX, u32::MIN), |(min, max), range| {
            (min.min(range.1), max.max(range.1))
        });
    // let min_accepted =

    println!("Min: {}, Max: {}", min + 1, max + 1);
}
