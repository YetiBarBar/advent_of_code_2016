use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedPosition(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split_ascii_whitespace().collect::<Vec<_>>();

        if splits[0] == "swap" {
            if splits[1] == "letter" {
                Ok(Self::SwapLetter(
                    splits[2].chars().next().unwrap(),
                    splits[5].chars().next().unwrap(),
                ))
            } else {
                Ok(Self::SwapPosition(
                    splits[2].parse().unwrap(),
                    splits[5].parse().unwrap(),
                ))
            }
        } else if splits[0] == "rotate" {
            if splits[1] == "left" {
                Ok(Self::RotateLeft(splits[2].parse().unwrap()))
            } else if splits[1] == "right" {
                Ok(Self::RotateRight(splits[2].parse().unwrap()))
            } else {
                Ok(Self::RotateBasedPosition(splits[6].chars().next().unwrap()))
            }
        } else if splits[0] == "reverse" {
            Ok(Self::Reverse(
                splits[2].parse().unwrap(),
                splits[4].parse().unwrap(),
            ))
        } else if splits[0] == "move" {
            Ok(Self::Move(
                splits[2].parse().unwrap(),
                splits[5].parse().unwrap(),
            ))
        } else {
            Err(())
        }
    }
}

fn apply_instruction(instruction: &Instruction, line: &[char]) -> Vec<char> {
    match instruction {
        Instruction::SwapLetter(l1, l2) => swap_letter(line, *l1, *l2),
        Instruction::SwapPosition(p1, p2) => {
            let mut ret = line.to_vec();
            ret[*p1] = line[*p2];
            ret[*p2] = line[*p1];
            ret
        }
        Instruction::RotateLeft(index) => rotate_left(line, *index),
        Instruction::RotateRight(index) => rotate_right(line, *index),
        Instruction::RotateBasedPosition(letter) => {
            let mut rotate_index = line
                .iter()
                .enumerate()
                .find(|(_, chr)| chr == &letter)
                .unwrap()
                .0;
            rotate_index += if rotate_index >= 4 { 2 } else { 1 };
            rotate_index %= line.len();
            rotate_right(line, rotate_index)
        }
        Instruction::Reverse(inf, sup) => {
            //let ret = line.clone();
            let reversed = line[*inf..=*sup].iter().rev();
            let mut ret = line[..*inf].to_vec();
            ret.extend(reversed);
            if ret.len() != line.len() {
                ret.extend(&line[*sup + 1..]);
            }
            ret
        }
        Instruction::Move(x, y) => move_position(line, *x, *y),
    }
}

fn move_position(line: &[char], x: usize, y: usize) -> Vec<char> {
    let moved_item = *line.get(x).unwrap();
    let filtered: Vec<char> = line
        .iter()
        .filter(|chr| chr != &&moved_item)
        .copied()
        .collect();
    let mut ret = Vec::with_capacity(line.len());

    for position in 0..line.len() {
        match position.cmp(&y) {
            std::cmp::Ordering::Less => {
                ret.push(filtered[position]);
            }
            std::cmp::Ordering::Equal => {
                ret.push(moved_item);
            }
            std::cmp::Ordering::Greater => {
                ret.push(filtered[position - 1]);
            }
        }
    }
    ret
}

fn rotate_left(line: &[char], index: usize) -> Vec<char> {
    let mut ret = line.to_vec();
    ret.rotate_left(index);
    ret
}

fn rotate_right(line: &[char], index: usize) -> Vec<char> {
    let mut ret = line.to_vec();
    ret.rotate_right(index);
    ret
}
fn swap_letter(line: &[char], l1: char, l2: char) -> Vec<char> {
    line.iter()
        .map(|&chr| {
            if chr == l1 {
                l2
            } else if chr == l2 {
                l1
            } else {
                chr
            }
        })
        .collect()
}

fn main() {
    let instructions: Vec<Instruction> = include_str!("../data/day_2016_21.data")
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let base_password = "abcdefgh".chars().collect::<Vec<char>>();
    let encoded = encoder(&base_password, instructions.clone());
    println!("Part 1: {encoded}");

    let permuts: Vec<_> = base_password.iter().permutations(8).collect();
    for permut in permuts {
        let permut = permut.iter().map(|chr| **chr).collect::<Vec<_>>();
        if encoder(&permut, instructions.clone()) == "fbgdceah" {
            println!("Part 2: {}", permut.iter().copied().collect::<String>());
            break;
        }
    }
}

fn encoder(password: &[char], instructions: Vec<Instruction>) -> String {
    let mut vector = password.to_vec();
    for instruction in instructions {
        vector = apply_instruction(&instruction, &vector);
        //     println!("{:?}", vector);
    }
    vector.iter().collect::<String>()
}
