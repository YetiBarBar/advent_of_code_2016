use std::str::FromStr;

fn main() {
    let input = include_str!("../data/day4.data");

    println!(
        "Valid lines: {}",
        input
            .lines()
            .map(str::parse)
            .map(Result::unwrap)
            .filter(Room::is_valid)
            .map(|room| room.sector_id)
            .sum::<usize>()
    );

    for line in input.lines() {
        let room: Room = line.parse().unwrap();
        if room.uncipher().contains("northpole") {
            println!("{} => {:?}", room.uncipher(), room);
            break;
        }
    }
}

#[derive(Debug)]
struct Room {
    code: String,
    checksum: String,
    sector_id: usize,
}

#[derive(PartialEq, Eq)]
struct Letter {
    value: char,
    count: usize,
}

impl Ord for Letter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count
            .cmp(&other.count)
            .then(self.value.cmp(&other.value).reverse())
    }
}

impl PartialOrd for Letter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Room {
    fn compute_cheksum(&self) -> String {
        let mut hmap = std::collections::HashMap::new();
        for chr in self.code.chars().filter(char::is_ascii_alphabetic) {
            *hmap.entry(chr).or_default() += 1;
        }
        let mut v = Vec::new();
        for (key, val) in hmap {
            v.push(Letter {
                value: key,
                count: val,
            });
        }
        v.sort();
        v.iter().rev().take(5).map(|item| item.value).collect()
    }

    fn is_valid(&self) -> bool {
        self.compute_cheksum() == self.checksum
    }

    fn uncipher(&self) -> String {
        let index = self.sector_id % 26;
        self.code
            .chars()
            .filter_map(|chr| {
                if chr.is_alphabetic() {
                    ('a'..='z')
                        .cycle()
                        .skip_while(|item| item != &chr)
                        .nth(index)
                } else if chr == '-' {
                    Some(' ')
                } else {
                    None
                }
            })
            .collect()
    }
}

impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (code, res) = s.split_once('[').ok_or(())?;
        let (checksum, _) = res.split_once(']').ok_or(())?;
        Ok(Room {
            code: code.into(),
            checksum: checksum.into(),
            sector_id: s
                .chars()
                .filter(|chr| chr.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap(),
        })
    }
}
