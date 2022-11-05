use std::collections::HashSet;

fn main() {
    let input = "bwnlcvfs";

    let res = bfs((0, 0), (3, 3), input);
    println!(
        "Step 1: {}",
        res.chars().skip(input.len()).collect::<String>()
    );
    println!("Step 2: {}", step2((0, 0), (3, 3), input));
}

fn adjacents(current: &HashSet<((usize, usize), String)>) -> HashSet<((usize, usize), String)> {
    let mut hset = HashSet::new();
    for ((pos_x, pos_y), word) in current {
        let digest = format!("{:?}", md5::compute(word.as_bytes()));
        let directions: Vec<_> = digest.chars().take(4).collect();
        // up, down, left, and right
        if directions[0].to_digit(16).unwrap() > 10 && pos_y > &0 {
            hset.insert(((*pos_x, *pos_y - 1), {
                let mut chain = word.clone();
                chain.push('U');
                chain
            }));
        }
        if directions[1].to_digit(16).unwrap() > 10 && pos_y < &3 {
            hset.insert(((*pos_x, *pos_y + 1), {
                let mut chain = word.clone();
                chain.push('D');
                chain
            }));
        }
        if directions[2].to_digit(16).unwrap() > 10 && pos_x > &0 {
            hset.insert(((*pos_x - 1, *pos_y), {
                let mut chain = word.clone();
                chain.push('L');
                chain
            }));
        }
        if directions[3].to_digit(16).unwrap() > 10 && pos_x < &3 {
            hset.insert(((*pos_x + 1, *pos_y), {
                let mut chain = word.clone();
                chain.push('R');
                chain
            }));
        }
    }
    hset
}

fn bfs(source: (usize, usize), destination: (usize, usize), pass: &str) -> String {
    let mut visited: HashSet<(usize, usize)> = [(source.0, source.1)].iter().copied().collect();
    let mut current: HashSet<((usize, usize), String)> = [(source.0, source.1)]
        .iter()
        .copied()
        .map(|source| (source, pass.to_string()))
        .collect();

    while !visited.iter().any(|item| &destination == item) {
        current = adjacents(&current);
        for pts in &current {
            visited.insert(pts.0);
        }

        if current.is_empty() {
            return "Not concluding!".to_string();
        }
    }

    current
        .iter()
        .find(|(pos, _)| pos == &destination)
        .unwrap()
        .1
        .clone()
}

fn step2(source: (usize, usize), destination: (usize, usize), pass: &str) -> usize {
    let mut steps = 0_usize;
    let mut longer = 0;
    let mut visited: HashSet<(usize, usize)> = [(source.0, source.1)].iter().copied().collect();
    let mut current: HashSet<((usize, usize), String)> = [(source.0, source.1)]
        .iter()
        .copied()
        .map(|source| (source, pass.to_string()))
        .collect();

    while !current.is_empty() {
        steps += 1;
        current = adjacents(&current);

        for pts in &current {
            if pts.0 == destination {
                longer = steps;
            } else {
                visited.insert(pts.0);
            }
        }
        current.retain(|item| item.0 != destination);
    }
    longer
}
