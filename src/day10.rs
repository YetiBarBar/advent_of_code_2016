use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
struct Bot {
    index: usize,
    values: Vec<usize>,
    destination_low: Destination,
    destination_high: Destination,
}

impl Bot {
    fn add_value(&mut self, value: usize) {
        self.values.push(value);
        self.values.sort_unstable();
        self.values.dedup();
    }
}

#[derive(Debug)]
struct FixedValue {
    value: usize,
    bot_index: usize,
}

impl FromStr for FixedValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split_ascii_whitespace().collect();
        let value = splits[1].parse().unwrap();
        let bot_index = splits[5].parse().unwrap();
        Ok(Self { value, bot_index })
    }
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("bot") {
            return Err(());
        }
        let splits = s.split_ascii_whitespace().collect::<Vec<_>>();
        let index = splits[1].parse::<usize>().unwrap();
        let low = splits[6].parse::<usize>().unwrap();
        let high = splits[11].parse::<usize>().unwrap();
        let destination_low = match splits.get(5) {
            Some(&"bot") => Destination::Bot(low),
            Some(&"output") => Destination::Output(low),
            _ => return Err(()),
        };
        let destination_high = match splits.get(10) {
            Some(&"bot") => Destination::Bot(high),
            Some(&"output") => Destination::Output(high),
            _ => return Err(()),
        };
        Ok(Self {
            index,
            values: Vec::new(),
            destination_low,
            destination_high,
        })
    }
}

#[derive(Debug, Clone)]
enum Destination {
    Bot(usize),
    Output(usize),
}

fn propagate(
    mut bots: HashMap<usize, Bot>,
    mut outputs: HashMap<usize, usize>,
) -> (HashMap<usize, Bot>, HashMap<usize, usize>) {
    let complete_bots: Vec<Bot> = bots
        .values()
        .filter(|b| b.values.len() > 1)
        .cloned()
        .collect();

    for bot in complete_bots {
        match bot.destination_low {
            Destination::Bot(index) => {
                bots.entry(index).and_modify(|b| b.add_value(bot.values[0]));
            }
            Destination::Output(index) => {
                *outputs.entry(index).or_default() = bot.values[0];
            }
        }

        match bot.destination_high {
            Destination::Bot(index) => {
                bots.entry(index).and_modify(|b| b.add_value(bot.values[1]));
            }
            Destination::Output(index) => {
                *outputs.entry(index).or_default() = bot.values[1];
            }
        };
    }

    (bots, outputs)
}

fn main() {
    let data = include_str!("../data/day_2016_10.data");
    let (bots, values) = data
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut b, mut v), line| {
            if let Ok(bot) = line.parse::<Bot>() {
                b.push(bot);
            } else {
                v.push(line.parse::<FixedValue>().unwrap());
            }
            (b, v)
        });

    let mut bots: HashMap<usize, Bot> = bots.into_iter().map(|b| (b.index, b)).collect();
    let mut outputs = HashMap::new();

    // Processing values
    for value in values {
        bots.get_mut(&value.bot_index)
            .unwrap()
            .add_value(value.value);
    }

    // Propagate all bots
    let mut exit_loop = false;
    loop {
        (bots, outputs) = propagate(bots, outputs);

        if bots.values().all(|b| b.values.len() == 2) {
            // We run the loop an extra time to eventually propagate
            // last values!
            exit_loop = true;
        }
        if exit_loop {
            break;
        }
    }

    println!(
        "Part 1: {}",
        bots.values()
            .find(|b| b.values.get(1) == Some(&61) && b.values.first() == Some(&17))
            .unwrap()
            .index
    );

    println!(
        "Part 2: {}",
        outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap()
    );
}
