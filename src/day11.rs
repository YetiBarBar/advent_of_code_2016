use itertools::Itertools;

use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Atom {
    Promethium,
    Cobalt,
    Curium,
    Plutonium,
    Ruthenium,
    Elerium,
    Dilirium,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Obj {
    Generator(Atom),
    Microchip(Atom),
}

impl Obj {
    fn is_generator(&self) -> bool {
        matches!(self, Obj::Generator(_))
    }

    fn is_microchip(&self) -> bool {
        matches!(self, Obj::Microchip(_))
    }

    fn get_kind(&self) -> Atom {
        match self {
            Obj::Generator(v) => v.clone(),
            Obj::Microchip(v) => v.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    elevator_position: usize,
    level_1: HashSet<Obj>,
    level_2: HashSet<Obj>,
    level_3: HashSet<Obj>,
    level_4: HashSet<Obj>,
}

fn is_level_ok(level: &HashSet<Obj>) -> bool {
    if level.iter().filter(|obj| obj.is_generator()).count() == 0 {
        return true;
    }
    level
        .iter()
        .filter(|obj| obj.is_microchip())
        .all(|obj| level.contains(&Obj::Generator(obj.get_kind())))
}

fn count_gen_chip(v: &HashSet<Obj>) -> (usize, usize) {
    (
        v.iter().filter(|item| item.is_generator()).count(),
        v.iter().filter(|item| item.is_microchip()).count(),
    )
}

impl State {
    fn is_winning(&self) -> bool {
        self.level_1.is_empty() && self.level_2.is_empty() && self.level_3.is_empty()
    }

    fn next_possible_state(&self) -> Vec<State> {
        match self.elevator_position {
            1 => self.next_state_elevator_1(),
            2 => self.next_state_elevator_2(),
            3 => self.next_state_elevator_3(),
            4 => self.next_state_elevator_4(),
            _ => unreachable!(),
        }
    }

    fn is_valid_state(&self) -> bool {
        is_level_ok(&self.level_1)
            && is_level_ok(&self.level_2)
            && is_level_ok(&self.level_3)
            && is_level_ok(&self.level_4)
    }

    fn next_state_elevator_1(&self) -> Vec<State> {
        let mut res = vec![];
        for permut in self
            .level_1
            .iter()
            .permutations(1)
            .chain(self.level_1.iter().permutations(2))
        {
            let mut level_1 = self.level_1.clone();
            level_1.retain(|obj| !permut.contains(&obj));
            let mut level_2 = self.level_2.clone();
            level_2.extend(permut.into_iter().cloned());
            let state = State {
                elevator_position: 2,
                level_1,
                level_2,
                level_3: self.level_3.clone(),
                level_4: self.level_4.clone(),
            };
            if state.is_valid_state() {
                res.push(state);
            }
        }
        res
    }

    fn next_state_elevator_2(&self) -> Vec<State> {
        let mut res = vec![];
        for permut in self
            .level_2
            .iter()
            .permutations(1)
            .chain(self.level_2.iter().permutations(2))
        {
            let mut level_2 = self.level_2.clone();
            level_2.retain(|obj| !permut.contains(&obj));
            let mut level_3 = self.level_3.clone();
            level_3.extend(permut.clone().into_iter().cloned());

            let state_a = State {
                elevator_position: 3,
                level_1: self.level_1.clone(),
                level_2: level_2.clone(),
                level_3,
                level_4: self.level_4.clone(),
            };
            if state_a.is_valid_state() {
                res.push(state_a);
            }
            if !self.level_1.is_empty() {
                for permut in self.level_2.iter().permutations(1) {
                    let mut level_2 = self.level_2.clone();
                    level_2.retain(|obj| !permut.contains(&obj));
                    let mut level_1 = self.level_1.clone();
                    level_1.extend(permut.into_iter().cloned());
                    let state_b = State {
                        elevator_position: 1,
                        level_1,
                        level_2,
                        level_3: self.level_3.clone(),
                        level_4: self.level_4.clone(),
                    };
                    if state_b.is_valid_state() {
                        res.push(state_b);
                    }
                }
            }
        }
        res
    }

    fn next_state_elevator_3(&self) -> Vec<State> {
        let mut res = vec![];
        for permut in self
            .level_3
            .iter()
            .permutations(1)
            .chain(self.level_3.iter().permutations(2))
        {
            let mut level_3 = self.level_3.clone();
            level_3.retain(|obj| !permut.contains(&obj));
            let mut level_4 = self.level_4.clone();
            level_4.extend(permut.clone().into_iter().cloned());
            let mut level_2 = self.level_2.clone();
            level_2.extend(permut.into_iter().cloned());
            let state_a = State {
                elevator_position: 4,
                level_1: self.level_1.clone(),
                level_2: self.level_2.clone(),
                level_3: level_3.clone(),
                level_4,
            };
            if state_a.is_valid_state() {
                res.push(state_a);
            }
            if !(self.level_1.is_empty() && self.level_2.is_empty()) {
                let state_b = State {
                    elevator_position: 2,
                    level_1: self.level_1.clone(),
                    level_2,
                    level_3,
                    level_4: self.level_4.clone(),
                };
                if state_b.is_valid_state() {
                    res.push(state_b);
                }
            }
        }
        res
    }

    fn next_state_elevator_4(&self) -> Vec<State> {
        let mut res = vec![];
        for permut in self.level_4.iter().permutations(1) {
            let mut level_4 = self.level_4.clone();
            level_4.retain(|obj| !permut.contains(&obj));
            let mut level_3 = self.level_3.clone();
            level_3.extend(permut.into_iter().cloned());
            let state = State {
                elevator_position: 3,
                level_1: self.level_1.clone(),
                level_2: self.level_2.clone(),
                level_3,
                level_4,
            };
            if state.is_valid_state() {
                res.push(state);
            }
        }
        res
    }

    fn is_equivalent(&self, other: &State) -> bool {
        self.elevator_position == other.elevator_position
            && count_gen_chip(&self.level_1) == count_gen_chip(&other.level_1)
            && count_gen_chip(&self.level_2) == count_gen_chip(&other.level_2)
            && count_gen_chip(&self.level_3) == count_gen_chip(&other.level_3)
            && count_gen_chip(&self.level_4) == count_gen_chip(&other.level_4)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "Elevator : {}\n4: {:?}\n3: {:?}\n2: {:?}\n1:{:?}",
                self.elevator_position, self.level_4, self.level_3, self.level_2, self.level_1
            )
        )
    }
}

fn dedup_equivalent_state(input: Vec<State>) -> Vec<State> {
    let mut output = vec![];
    for item in input.into_iter() {
        if output.iter().all(|val: &State| !val.is_equivalent(&item)) {
            output.push(item);
        }
    }
    output
}

fn main() {
    /******************
     * The first floor contains a promethium generator and a promethium-compatible microchip.
     * The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
     * The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
     * The fourth floor contains nothing relevant.
     */
    use Atom::*;
    use Obj::*;
    let initial_state = State {
        level_1: [Generator(Promethium), Microchip(Promethium)]
            .iter()
            .cloned()
            .collect(),
        level_2: [
            Generator(Cobalt),
            Generator(Curium),
            Generator(Ruthenium),
            Generator(Plutonium),
        ]
        .iter()
        .cloned()
        .collect(),
        level_3: [
            Microchip(Cobalt),
            Microchip(Curium),
            Microchip(Ruthenium),
            Microchip(Plutonium),
        ]
        .iter()
        .cloned()
        .collect(),
        level_4: HashSet::new(),
        elevator_position: 1,
    };

    let mut current = Vec::new();
    current.push(initial_state.clone());

    let mut steps = 0;

    loop {
        steps += 1;
        current = current
            .iter()
            .flat_map(|state| state.next_possible_state())
            .collect();

        current = dedup_equivalent_state(current);

        if current.iter().any(|state| state.is_winning()) {
            break;
        }
    }

    // TODO: Find error, I have 35 instead of 33...
    println!("There is an error for part 1...");
    println!("Part 1: {}", steps);

    let initial_state_2 = State {
        level_1: [
            Generator(Promethium),
            Microchip(Promethium),
            Generator(Elerium),
            Microchip(Elerium),
            Generator(Dilirium),
            Microchip(Dilirium),
        ]
        .iter()
        .cloned()
        .collect(),
        level_2: [
            Generator(Cobalt),
            Generator(Curium),
            Generator(Ruthenium),
            Generator(Plutonium),
        ]
        .iter()
        .cloned()
        .collect(),
        level_3: [
            Microchip(Cobalt),
            Microchip(Curium),
            Microchip(Ruthenium),
            Microchip(Plutonium),
        ]
        .iter()
        .cloned()
        .collect(),
        level_4: HashSet::new(),
        elevator_position: 1,
    };

    let mut current = Vec::new();
    current.push(initial_state_2.clone());

    let mut steps = 0;

    loop {
        steps += 1;
        current = current
            .iter()
            .flat_map(|state| state.next_possible_state())
            .collect();

        current = dedup_equivalent_state(current);

        if current.iter().any(|state| state.is_winning()) {
            break;
        }
    }

    println!("Part 2: {}", steps);
}
