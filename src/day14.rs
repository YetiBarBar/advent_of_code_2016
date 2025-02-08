use std::collections::HashMap;

struct OtpGen {
    stack: HashMap<usize, Vec<char>>,
    index: usize,
    seed: String,
    stretching: bool,
}

impl OtpGen {
    fn new(seed: &str, stretching: bool) -> Self {
        let index = 0;

        Self {
            index,
            stack: HashMap::new(),
            seed: seed.to_string(),
            stretching,
        }
    }

    fn get_hash(&mut self, index: usize) -> &Vec<char> {
        if self.stretching {
            let mut data = format!("{}{}", self.seed, index);
            for _ in 0..=2016 {
                data = format!("{:?}", md5::compute(data.as_bytes()));
            }

            self.stack
                .entry(index)
                .or_insert_with(|| data.chars().collect())
        } else {
            self.stack.entry(index).or_insert_with(|| {
                let raw = format!("{}{}", self.seed, index);
                let data = format!("{:?}", md5::compute(raw.as_bytes()));
                data.chars().collect()
            })
        }
    }

    fn triplet(&mut self) -> Option<char> {
        self.get_hash(self.index)
            .windows(3)
            .find(|win| win[0] == win[1] && win[1] == win[2])
            .map(|win| win[0])
    }

    fn has_five_in_row(&mut self, val: char) -> bool {
        (self.index + 1..self.index + 1001).any(|idx| {
            self.get_hash(idx).windows(5).any(|win| {
                win[0] == win[1]
                    && win[1] == win[2]
                    && win[2] == win[3]
                    && win[3] == win[4]
                    && win[0] == val
            })
        })
    }
}

impl Iterator for OtpGen {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            println!("{}", self.index);
            if let Some(chr) = self.triplet() {
                if self.has_five_in_row(chr) {
                    self.index += 1;
                    return Some(self.index - 1);
                }
            }
            self.index += 1;
        }
    }
}

fn main() {
    let seed = "ahsbgdzn";
    let mut opt_gen = OtpGen::new(seed, false);
    // 63 because we need to count 0!
    let res = opt_gen.nth(63).unwrap();
    println!("Part 1: {res}");

    let mut opt_gen2 = OtpGen::new(seed, true);
    let res2 = opt_gen2.nth(63).unwrap();
    println!("Part 2: {res2}");
}
