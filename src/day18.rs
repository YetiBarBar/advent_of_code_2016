#[derive(Clone)]
struct TrapLine {
    last_line: String,
}

impl TrapLine {
    fn new(line: &str) -> Self {
        Self {
            last_line: line.into(),
        }
    }
}

impl Iterator for TrapLine {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.last_line.clone();
        let chars: Vec<_> = std::iter::once('.')
            .chain(self.last_line.chars())
            .chain(std::iter::once('.'))
            .collect();

        self.last_line = chars
            .windows(3)
            .map(|win| match (win[0], win[1], win[2]) {
                ('^', '^' | '.', '.') | ('.', '.' | '^', '^') => '^',
                _ => '.',
            })
            .collect();
        Some(to_return)
    }
}

fn main() {
    let trap_line = TrapLine::new(".^^.^^^..^.^..^.^^.^^^^.^^.^^...^..^...^^^..^^...^..^^^^^^..^.^^^..^.^^^^.^^^.^...^^^.^^.^^^.^.^^.^.");

    let res: Vec<String> = trap_line.clone().take(40).collect();
    assert_eq!(res.len(), 40);
    let count = res
        .iter()
        .map(|line| line.chars().filter(|chr| chr == &'.').count())
        .sum::<usize>();
    println!("Part 1: {}", count);

    let res2: Vec<String> = trap_line.take(400_000).collect();
    assert_eq!(res2.len(), 400_000);
    let count = res2
        .iter()
        .map(|line| line.chars().filter(|chr| chr == &'.').count())
        .sum::<usize>();
    println!("Part 2: {}", count);
}
