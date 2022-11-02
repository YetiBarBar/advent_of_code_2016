use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy)]
enum Instructions {
    Cpy(Value, char),
    Inc(char),
    Dec(char),
    Jnz(Value, isize),
}
struct Cpu {
    prog: Vec<Instructions>,
    register: HashMap<char, isize>,
    eip: isize,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Register(char),
    Integer(isize),
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split_ascii_whitespace().collect();
        match splits[0] {
            "inc" => Ok(Self::Inc(splits[1].chars().next().unwrap())),
            "dec" => Ok(Self::Dec(splits[1].chars().next().unwrap())),
            "cpy" => {
                let v = if let Ok(int_val) = splits[1].parse::<isize>() {
                    Value::Integer(int_val)
                } else {
                    Value::Register(splits[1].chars().next().unwrap())
                };
                Ok(Self::Cpy(v, splits[2].chars().next().unwrap()))
            }
            "jnz" => {
                let v = if let Ok(int_val) = splits[1].parse::<isize>() {
                    Value::Integer(int_val)
                } else {
                    Value::Register(splits[1].chars().next().unwrap())
                };
                Ok(Self::Jnz(v, splits[2].parse::<isize>().unwrap()))
            }
            _ => Err(()),
        }
    }
}

impl Cpu {
    fn new(prog: &[Instructions]) -> Self {
        Self {
            prog: prog.to_vec(),
            register: HashMap::new(),
            eip: 0,
        }
    }

    fn set_register(&mut self, reg: char, val: isize) {
        self.register.insert(reg, val);
    }

    fn inc(&mut self, register: char) {
        *self.register.entry(register).or_default() += 1;
        self.eip += 1;
    }

    fn dec(&mut self, register: char) {
        *self.register.entry(register).or_default() -= 1;
        self.eip += 1;
    }

    fn cpy(&mut self, src: Value, dest: char) {
        let v = match src {
            Value::Register(reg) => *self.register.entry(reg).or_default(),
            Value::Integer(ival) => ival,
        };
        *self.register.entry(dest).or_default() = v;
        self.eip += 1;
    }

    fn jnz(&mut self, cond: Value, offset: isize) {
        let jump = match cond {
            Value::Register(reg) => *self.register.entry(reg).or_default() != 0,
            Value::Integer(i) => i != 0,
        };
        if jump {
            self.eip += offset;
        } else {
            self.eip += 1;
        }
    }

    fn run_instrcution(&mut self, instr: &Instructions) {
        match instr {
            Instructions::Cpy(x, y) => self.cpy(*x, *y),
            Instructions::Inc(reg) => self.inc(*reg),
            Instructions::Dec(reg) => self.dec(*reg),
            Instructions::Jnz(x, offset) => self.jnz(*x, *offset),
        }
    }

    fn run_prog(&mut self) {
        while let Some(&instr) = self.prog.get(self.eip as usize) {
            self.run_instrcution(&instr);
        }
    }
}

fn main() {
    let input = include_str!("../data/day_2016_12.data")
        .lines()
        .map(str::parse::<Instructions>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut cpu = Cpu::new(&input);
    cpu.run_prog();
    println!("Part 1: {}", cpu.register.get(&'a').unwrap());

    let mut cpu = Cpu::new(&input);
    cpu.set_register('c', 1);
    cpu.run_prog();
    println!("Part 2: {}", cpu.register.get(&'a').unwrap());
}
