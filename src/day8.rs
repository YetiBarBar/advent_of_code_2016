use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
enum Command {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

struct AocDisplay {
    field: [[bool; 50]; 6],
}

impl AocDisplay {
    fn new() -> Self {
        Self {
            field: [[false; 50]; 6],
        }
    }

    fn rect(&mut self, x: usize, y: usize) {
        for cur_y in 0..y {
            for cur_x in 0..x {
                self.field[cur_y][cur_x] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, value: usize) {
        self.field[row].rotate_right(value);
    }

    fn rotate_col(&mut self, col: usize, value: usize) {
        let mut column: Vec<bool> = self.field.iter().map(|row| row[col]).collect();
        column.rotate_right(value);
        for (slot, val) in self.field.iter_mut().zip(column.iter()) {
            slot[col] = *val;
        }
    }

    fn run_cmd(&mut self, command: &[Command]) {
        for cmd in command {
            match cmd {
                Command::Rect(x, y) => self.rect(*x, *y),
                Command::RotateRow(row, val) => self.rotate_row(*row, *val),
                Command::RotateCol(col, val) => self.rotate_col(*col, *val),
            }
        }
    }

    fn count_pixels_on(&self) -> usize {
        self.field
            .iter()
            .flat_map(|row| row.iter())
            .filter(|v| **v)
            .count()
    }
}

impl Display for AocDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in self.field {
            for point in row {
                if point {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        write!(f, "{result}")
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("rect ") {
            let (_, value) = s.split_once(' ').unwrap();
            let (x, y) = value.split_once('x').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            return Ok(Self::Rect(x, y));
        }
        if s.starts_with("rotate row y=") {
            let splits = s.split_ascii_whitespace().collect::<Vec<_>>();
            let row = splits[2]
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap();
            let val = splits[4].parse().unwrap();
            return Ok(Self::RotateRow(row, val));
        }
        if s.starts_with("rotate column x=") {
            let splits = s.split_ascii_whitespace().collect::<Vec<_>>();
            let col = splits[2]
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap();
            let val = splits[4].parse().unwrap();
            return Ok(Self::RotateCol(col, val));
        }
        println!("Wrong line: {s}");
        Err(())
    }
}

fn main() {
    let commands: Vec<Command> = include_str!("../data/day_2016_8.data")
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::parse::<Command>)
        .map(Result::unwrap)
        .collect();

    let mut disp = AocDisplay::new();
    disp.run_cmd(&commands);
    println!("Part 1: {}", disp.count_pixels_on());
    println!("For part 2, need to read this:");
    println!("{disp}");
}
