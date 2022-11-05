use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<Vec<char>> = include_str!("../data/day_2016_24.data")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    //     let input: Vec<Vec<char>> = r#"###########
    // #0.1.....2#
    // #.#######.#
    // #4.......3#
    // ###########"#
    //        .lines()
    //        .map(|line| line.chars().collect())
    //        .collect();

    let points: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, chr)| *chr != &'.' && *chr != &'#')
                .map(move |(x, chr)| (chr, x, y))
        })
        .collect();

    println!("Points: {:?}", points);

    let mut distances = HashMap::new();
    // Compute all distance between points
    for idx_a in 0..points.len() {
        let mut dp = HashMap::new();
        for idx_b in 0..points.len() {
            if idx_b != idx_a {
                dp.insert(
                    *points[idx_b].0,
                    bfs(
                        (points[idx_b].1, points[idx_b].2),
                        (points[idx_a].1, points[idx_a].2),
                        &input,
                    ),
                );
            }
        }
        distances.insert(*points[idx_a].0, dp);
    }

    for line in distances.values() {
        println!("{:?}", &line);
    }
    // Permutes all  paths (0 is excluded)
    let permuts: Vec<_> = ('1'..)
        .take(points.len() - 1)
        .permutations(points.len() - 1)
        .collect();

    let res = permuts
        .iter()
        .map(|permutation| eval_permut(permutation, &distances, false))
        .min()
        .unwrap();
    println!("Part 1: {}", res);

    let res = permuts
        .iter()
        .map(|permutation| eval_permut(permutation, &distances, true))
        .min()
        .unwrap();
    println!("Part 2: {}", res);
}

fn eval_permut(
    per: &[char],
    distances: &HashMap<char, HashMap<char, usize>>,
    return_to_base: bool,
) -> usize {
    let mut v = vec!['0'];
    v.extend(per.iter().copied());

    if return_to_base {
        v.push('0');
    }
    v.windows(2)
        .map(|win| distances.get(&win[0]).unwrap().get(&win[1]).unwrap())
        .sum()
}

fn is_wall(x: usize, y: usize, marble: &[Vec<char>]) -> bool {
    if let Some(row) = marble.get(y) {
        row.get(x).unwrap_or(&'#') == &'#'
    } else {
        true
    }
}

fn adjacents(visited: &HashSet<(usize, usize)>, marble: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut hset = HashSet::new();
    for &(pos_x, pos_y) in visited {
        if pos_x != 0 && !is_wall(pos_x - 1, pos_y, marble) {
            hset.insert((pos_x - 1, pos_y));
        }
        if pos_y != 0 && !is_wall(pos_x, pos_y - 1, marble) {
            hset.insert((pos_x, pos_y - 1));
        }
        if !is_wall(pos_x, pos_y + 1, marble) {
            hset.insert((pos_x, pos_y + 1));
        }
        if !is_wall(pos_x + 1, pos_y, marble) {
            hset.insert((pos_x + 1, pos_y));
        }
    }
    hset
}

fn bfs(source: (usize, usize), destination: (usize, usize), marble: &[Vec<char>]) -> usize {
    let mut steps = 0_usize;
    let mut visited: HashSet<(usize, usize)> = [source].iter().copied().collect();
    let mut current: HashSet<(usize, usize)>;

    while !visited.contains(&destination) {
        steps += 1;
        current = adjacents(&visited, marble);
        for pts in current {
            visited.insert(pts);
        }
    }
    steps
}
