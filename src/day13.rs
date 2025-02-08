use std::collections::HashSet;

fn is_wall(x: isize, y: isize, fav: isize) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + fav).count_ones() % 2 == 1 || x < 0 || y < 0
}

fn adjacents(visited: &HashSet<(isize, isize)>, fav: isize) -> HashSet<(isize, isize)> {
    visited
        .iter()
        .flat_map(|(pos_x, pos_y)| {
            [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .iter()
                .map(move |(dx, dy)| (pos_x + dx, pos_y + dy))
                .filter(|(x, y)| !is_wall(*x, *y, fav))
        })
        .collect()
}

fn main() {
    let mut steps = 0_usize;
    let mut visited: HashSet<(isize, isize)> = [(1, 1)].iter().copied().collect();
    let mut current: HashSet<(isize, isize)>;

    while !visited.contains(&(31, 39)) {
        steps += 1;
        current = adjacents(&visited, 1350);
        if steps == 50 {
            println!("Part 2: {}", current.len());
        }
        for pts in current {
            visited.insert(pts);
        }
    }

    println!("Part 1: {steps}");
}
