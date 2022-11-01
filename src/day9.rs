#[derive(Debug)]
struct ExpandLen {
    value: Vec<char>,
    times: usize,
    total_len: usize,
}

impl ExpandLen {
    fn new(input: &str) -> Self {
        // Skip opening parens
        let char_iter = input.chars().skip(1);
        let len = char_iter
            .clone()
            .take_while(|ch| ch != &'x')
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let times = char_iter
            .clone()
            .skip_while(|chr| chr != &'x')
            .skip(1)
            .take_while(|chr| chr != &')')
            .collect::<String>()
            .parse()
            .unwrap();
        let value = char_iter
            .clone()
            .skip_while(|chr| chr != &')')
            .skip(1)
            .take(len)
            .collect();
        let total_len = char_iter.clone().take_while(|chr| chr != &')').count() + 2 + len;
        Self {
            value,
            times,
            total_len,
        }
    }

    fn expand(&self) -> usize {
        self.value.len() * self.times
    }
}

fn expand_len(data: &str) -> usize {
    let mut min_position = 0;
    let opening_parens = data
        .chars()
        .enumerate()
        .filter_map(|(pos, chr)| (chr == '(').then(|| pos))
        .collect::<Vec<_>>();

    let closing_parens = data
        .chars()
        .enumerate()
        .filter_map(|(pos, chr)| (chr == ')').then(|| pos))
        .collect::<Vec<_>>();

    let mut res = 0;
    for position in opening_parens {
        if min_position > position {
            // res += 0;
            ()
        } else {
            if min_position < position {
                res += position - min_position;
            }

            let expander = ExpandLen::new(&data[position..]);
            res += expander.expand();
            min_position = position + expander.total_len;
        }
    }
    res += data.len() - min_position;
    res
}

fn main() {
    let data = include_str!("../data/day_2016_9.data");
    let res: usize = data.lines().map(|line| expand_len(line)).sum();
    println!("Part 1: {}", res);
}
