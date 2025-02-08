struct Compressor {
    start: usize,
    end: usize,
    payload: String,
    times: usize,
}

impl Compressor {
    fn new(input: &str, start: usize, end: usize) -> Self {
        let (len_to_catch, times) = input[start + 1..end]
            .split_once('x')
            .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
            .unwrap();
        let payload = input.chars().skip(end + 1).take(len_to_catch).collect();
        Self {
            start,
            end,
            payload,
            times,
        }
    }

    fn len(&self, recurse: bool) -> usize {
        if recurse {
            self.times * estimate_len(&self.payload, true)
        } else {
            self.times * self.payload.len()
        }
    }
}

fn estimate_len(data: &str, recurse: bool) -> usize {
    let compressors = data
        .chars()
        .enumerate()
        .filter_map(|(pos, chr)| (chr == '(').then_some(pos))
        .zip(
            data.chars()
                .enumerate()
                .filter_map(|(pos, chr)| (chr == ')').then_some(pos)),
        )
        .map(|(inf, sup)| Compressor::new(data, inf, sup))
        .collect::<Vec<_>>();

    let mut cursor = 0_usize;
    let mut len = 0;
    let data: Vec<char> = data.chars().collect();
    while cursor != data.len() {
        if data[cursor] == '(' {
            let compressor = compressors.iter().find(|c| c.start == cursor).unwrap();
            len += compressor.len(recurse);
            cursor = compressor.payload.len() + compressor.end + 1;
        } else {
            len += 1;
            cursor += 1;
        }
    }
    len
}

fn main() {
    let data = include_str!("../data/day_2016_9.data");
    let res: usize = data.lines().map(|line| estimate_len(line, false)).sum();
    println!("Part 1: {res}");
    let res2: usize = data.lines().map(|line| estimate_len(line, true)).sum();
    println!("Part 1: {res2}");
}
