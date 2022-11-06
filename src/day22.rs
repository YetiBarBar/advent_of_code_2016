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
        // /dev/grid/node-x0-y9     88T   69T    19T   78%
        // Size  Used  Avail  Use%
        let splits: Vec<_> = s.split_ascii_whitespace().collect();
        let x = splits[0]
            .chars()
            .skip_while(|chr| chr != &'x')
            .skip(1)
            .take_while(|x| x.is_ascii_digit())
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
        .map(str::parse)
        .filter(|res| res.is_ok())
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {}", viable_pairs(&input));
}
