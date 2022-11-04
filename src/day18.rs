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

    println!("Part 1: {}", process(&trap_line, 40));
    println!("Part 2: {}", process(&trap_line, 400_000));
}

fn process(trap_line: &TrapLine, iteration: usize) -> usize {
    let res: Vec<String> = trap_line.clone().take(iteration).collect();
    let count = res
        .iter()
        .map(|line| line.chars().filter(|chr| chr == &'.').count())
        .sum::<usize>();
    count
}
