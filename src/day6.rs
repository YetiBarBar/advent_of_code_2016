use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../data/day6.data");
    let mut v = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    for line in input.lines() {
        for (position, chr) in line.chars().enumerate() {
            let hmap = v.get_mut(position).unwrap();
            *hmap.entry(chr).or_default() += 1;
        }
    }

    let result = v.iter().map(get_max).collect::<String>();
    println!("{}", result);
    let result2 = v.iter().map(get_min).collect::<String>();
    println!("{}", result2);
}

fn get_max(hmap: &HashMap<char, usize>) -> char {
    *hmap
        .iter()
        .reduce(|acc, (key, val)| if val > acc.1 { (key, val) } else { acc })
        .unwrap()
        .0
}
fn get_min(hmap: &HashMap<char, usize>) -> char {
    *hmap
        .iter()
        .reduce(|acc, (key, val)| if val < acc.1 { (key, val) } else { acc })
        .unwrap()
        .0
}
