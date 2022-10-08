pub struct LetterGen {
    base: String,
    count: usize,
    goal: String,
}

impl Iterator for LetterGen {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        for idx in self.count.. {
            let data = format!("{}{}", self.base, idx);
            let hashed = format!("{:?}", md5::compute(data.as_bytes()));
            if hashed.starts_with(&self.goal) {
                self.count = idx + 1;
                return hashed.chars().nth(5);
            }
        }
        None
    }
}

pub struct LetterGen2 {
    base: String,
    count: usize,
    goal: String,
    internal_v: [Option<char>; 8],
    reached: usize,
}

impl LetterGen2 {
    fn process(&mut self) -> String {
        while self.internal_v.iter().any(Option::is_none) {
            let data = format!("{}{}", self.base, self.count);
            self.count += 1;
            let hashed = format!("{:?}", md5::compute(data.as_bytes()));
            if hashed.starts_with(&self.goal) {
                if let Some(position) = hashed.chars().nth(5).and_then(|s| s.to_digit(10)) {
                    if position < 8 && self.internal_v[position as usize].is_none() {
                        self.reached += 1;
                        self.internal_v[position as usize] = hashed.chars().nth(6);
                    }
                }
            }
        }
        self.internal_v.iter().map(|c| c.unwrap()).collect()
    }
}

pub fn main() {
    // let p1 = "abc";
    let p1 = "ffykfhsq";
    let lettergen = LetterGen {
        base: p1.into(),
        goal: "00000".into(),
        count: 0,
    };
    let mut lettergen2 = LetterGen2 {
        base: p1.into(),
        goal: "00000".into(),
        count: 0,
        reached: 0,
        internal_v: [None, None, None, None, None, None, None, None],
    };
    println!("Step 1: {}", lettergen.take(8).collect::<String>());
    println!("Step 2: {}", &lettergen2.process());
}
