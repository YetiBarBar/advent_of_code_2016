use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Partition {
    available: usize,
    size: usize,
    x: usize,
    y: usize,
    used: usize,
    perc_use: usize,
}

impl FromStr for Partition {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split_ascii_whitespace().collect();
        let x = splits[0]
            .chars()
            .skip_while(|chr| chr != &'x')
            .skip(1)
            .take_while(char::is_ascii_digit)
            .collect::<String>();

        let x = x.parse::<usize>()?;

        let y = splits[0]
            .chars()
            .skip_while(|chr| chr != &'y')
            .skip(1)
            .collect::<String>()
            .parse::<usize>()?;
        let size = splits[1].trim_end_matches('T').parse()?;
        let used = splits[2].trim_end_matches('T').parse()?;
        let available = splits[3].trim_end_matches('T').parse()?;
        let perc_use = splits[4].trim_end_matches('%').parse()?;
        Ok(Self {
            available,
            size,
            x,
            y,
            used,
            perc_use,
        })
    }
}

fn viable_pairs(partitions: &[Partition]) -> usize {
    let permuts = partitions.iter().permutations(2);
    permuts
        .filter(|permut| permut[0].used != 0 && permut[0].used < permut[1].available)
        .count()
}

fn main() {
    let input: Vec<Partition> = include_str!("../data/day_2016_22.data")
        .lines()
        .flat_map(str::parse)
        .collect();

    println!("Part 1: {}", viable_pairs(&input));

    // Todo: put BFS code for part 2 (or solve it manually!)
}
