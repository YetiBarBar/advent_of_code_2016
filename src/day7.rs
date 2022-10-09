use std::str::FromStr;

#[derive(Debug)]
struct PacketV7 {
    fields: Vec<String>,
    fields_abba: Vec<String>,
}

impl FromStr for PacketV7 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = Vec::new();
        let mut fields_abba = Vec::new();
        for (parity, part) in s.split(|chr| chr == '[' || chr == ']').enumerate() {
            if parity % 2 == 0 {
                fields.push(part.to_string());
            } else {
                fields_abba.push(part.to_string());
            }
        }
        Ok(Self {
            fields,
            fields_abba,
        })
    }
}

fn is_abba(seq: &str) -> bool {
    let seq = seq.chars().collect::<Vec<_>>();
    seq.windows(4)
        .any(|a| a[0] == a[3] && a[1] == a[2] && a[0] != a[1])
}

impl PacketV7 {
    fn is_tls(&self) -> bool {
        self.fields_abba.iter().all(|v| !is_abba(v)) && self.fields.iter().any(|s| is_abba(s))
    }

    fn is_ssl(&self) -> bool {
        for field in &self.fields {
            let field: Vec<_> = field.chars().collect();
            for window in field.windows(3) {
                if window[0] == window[2] {
                    let seq = format!("{}{}{}", window[1], window[0], window[1]);
                    if self.fields_abba.iter().any(|s| s.contains(&seq)) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub fn main() {
    // Parsing is done assuming the [] are always open and properly closed
    // AND packets do not start with []
    let input = include_str!("../data/day7.data");

    let res = input
        .lines()
        .map(PacketV7::from_str)
        .map(Result::unwrap)
        .filter(PacketV7::is_tls)
        .count();
    println!("Step 1: {}", res);

    let res2 = input
        .lines()
        .map(PacketV7::from_str)
        .map(Result::unwrap)
        .filter(PacketV7::is_ssl)
        .count();
    println!("Step 2: {}", res2);
}
