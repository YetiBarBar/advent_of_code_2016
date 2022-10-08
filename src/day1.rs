use std::{collections::HashSet, str::FromStr};

fn main() {
    //let raw_data = "R1, R3, L2, L5, L2, L1, R3, L4, R2, L2, L4, R2, L1, R1, L2, R3, L1, L4, R2, L5, R3, R4, L1, R2, L1, R3, L4, R5, L4, L5, R5, L3, R2, L3, L3, R1, R3, L4, R2, R5, L4, R1, L1, L1, R5, L2, R1, L2, R188, L5, L3, R5, R1, L2, L4, R3, R5, L3, R3, R45, L4, R4, R72, R2, R3, L1, R1, L1, L1, R192, L1, L1, L1, L4, R1, L2, L5, L3, R5, L3, R3, L4, L3, R1, R4, L2, R2, R3, L5, R3, L1, R1, R4, L2, L3, R1, R3, L4, L3, L4, L2, L2, R1, R3, L5, L1, R4, R2, L4, L1, R3, R3, R1, L5, L2, R4, R4, R2, R1, R5, R5, L4, L1, R5, R3, R4, R5, R3, L1, L2, L4, R1, R4, R5, L2, L3, R4, L4, R2, L2, L4, L2, R5, R1, R4, R3, R5, L4, L4, L5, L5, R3, R4, L1, L3, R2, L2, R1, L3, L5, R5, R5, R3, L4, L2, R4, R5, R1, R4, L3";
    // let raw_data = "R5, L5, R5, R3";
    // let raw_data = "L5, R5, L5, L3";
    //let raw_data = "R8, R4, R4, R8";
    let raw_data = "R1, R3, L2, L5, L2, L1, R3, L4, R2, L2, L4, R2, L1, R1, L2, R3, L1, L4, R2, L5, R3, R4, L1, R2, L1, R3, L4, R5, L4, L5, R5, L3, R2, L3, L3, R1, R3, L4, R2, R5, L4, R1, L1, L1, R5, L2, R1, L2, R188, L5, L3, R5, R1, L2, L4, R3, R5, L3, R3, R45, L4, R4, R72, R2, R3, L1, R1, L1, L1, R192, L1, L1, L1, L4, R1, L2, L5, L3, R5, L3, R3, L4, L3, R1, R4, L2, R2, R3, L5, R3, L1, R1, R4, L2, L3, R1, R3, L4, L3, L4, L2, L2, R1, R3, L5, L1, R4, R2, L4, L1, R3, R3, R1, L5, L2, R4, R4, R2, R1, R5, R5, L4, L1, R5, R3, R4, R5, R3, L1, L2, L4, R1, R4, R5, L2, L3, R4, L4, R2, L2, L4, L2, R5, R1, R4, R3, R5, L4, L4, L5, L5, R3, R4, L1, L3, R2, L2, R1, L3, L5, R5, R5, R3, L4, L2, R4, R5, R1, R4, L3";

    let actions: Result<Vec<Instructions>, _> = raw_data.split(", ").map(str::parse).collect();
    let actions = actions.unwrap();

    let rob = Robot::new(0, 0, Direction::North);
    let rob = rob.follow_instructions(&actions);
    println!("{}", &rob);
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq)]
enum Orientation {
    Left,
    Right,
}

struct Instructions {
    orient: Orientation,
    moves: i32,
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let orient = match s.chars().next() {
            Some('L') => Orientation::Left,
            Some('R') => Orientation::Right,
            _ => return Err(()),
        };

        let moves = s[1..].parse().map_err(|_| ())?;

        Ok(Instructions { orient, moves })
    }
}

impl Direction {
    #[must_use]
    pub fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    #[must_use]
    pub fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
    visited: std::collections::HashSet<(i32, i32)>,
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: ({} , {}), Direction: {:?}",
            self.x, self.y, &self.d
        )
    }
}

impl Robot {
    #[must_use]
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x,
            y,
            d,
            visited: HashSet::default(),
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_d = self.d.right();
        Robot { d: new_d, ..self }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_d = self.d.left();
        Robot { d: new_d, ..self }
    }

    #[must_use]
    pub fn advance(mut self, times: i32) -> Self {
        let (dx, dy) = match self.d {
            Direction::North => (0, times),
            Direction::South => (0, -times),
            Direction::East => (times, 0),
            Direction::West => (-times, 0),
        };

        for i in 1..=dx {
            if self.visited.contains(&(self.x + i, self.y)) {
                println!("Already visited: {} {}", self.x + i, self.y);
            } else {
                self.visited.insert((self.x + i, self.y));
            }
        }

        for i in 1..=dy {
            if self.visited.contains(&(self.x, self.y + i)) {
                println!("Already visited: {} {}", self.x, self.y + i);
            } else {
                self.visited.insert((self.x, self.y + i));
            }
        }

        self.x += dx;
        self.y += dy;

        self
    }

    #[must_use]
    fn follow_instructions(self, instructions: &[Instructions]) -> Self {
        instructions.iter().fold(self, |rob, instr| {
            let rob = if instr.orient == Orientation::Left {
                rob.turn_left()
            } else {
                rob.turn_right()
            };
            rob.advance(instr.moves)
        })
    }
}
