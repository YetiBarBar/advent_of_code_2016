use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy, Debug)]
enum Instructions {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
    Out(Value),
}
struct Cpu {
    prog: Vec<Instructions>,
    register: HashMap<char, isize>,
    eip: isize,
    output: Vec<isize>,
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
        match splits.first() {
            Some(&"inc") => Ok(Self::Inc(Value::Register(
                splits[1].chars().next().unwrap(),
            ))),
            Some(&"dec") => Ok(Self::Dec(Value::Register(
                splits[1].chars().next().unwrap(),
            ))),
            Some(&"cpy") => {
                let v = if let Ok(int_val) = splits[1].parse::<isize>() {
                    Value::Integer(int_val)
                } else {
                    Value::Register(splits[1].chars().next().unwrap())
                };
                Ok(Self::Cpy(
                    v,
                    Value::Register(splits[2].chars().next().unwrap()),
                ))
            }
            Some(&"jnz") => {
                let v1 = if let Ok(int_val) = splits[1].parse::<isize>() {
                    Value::Integer(int_val)
                } else {
                    Value::Register(splits[1].chars().next().unwrap())
                };
                let v2 = if let Ok(int_val) = splits[2].parse::<isize>() {
                    Value::Integer(int_val)
                } else {
                    Value::Register(splits[2].chars().next().unwrap())
                };
                Ok(Self::Jnz(v1, v2))
            }
            Some(&"tgl") => {
                let v = if let Ok(int_val) = splits[1].parse::<isize>() {
                    Value::Integer(int_val)
                } else {
                    Value::Register(splits[1].chars().next().unwrap())
                };
                Ok(Self::Tgl(v))
            }
            Some(&"out") => {
                let v = if let Ok(int_val) = splits[1].parse::<isize>() {
                    Value::Integer(int_val)
                } else {
                    Value::Register(splits[1].chars().next().unwrap())
                };
                Ok(Self::Out(v))
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
            output: Vec::new(),
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

    fn jnz(&mut self, cond: Value, offset: Value) {
        let jump = match cond {
            Value::Register(reg) => *self.register.entry(reg).or_default() != 0,
            Value::Integer(i) => i != 0,
        };
        let offset = match offset {
            Value::Register(reg) => *self.register.entry(reg).or_default(),
            Value::Integer(i) => i,
        };
        if jump {
            self.eip += offset;
        } else {
            self.eip += 1;
        }
    }

    fn toggle(&mut self, offset: Value) {
        let offset = match offset {
            Value::Register(reg) => *self.register.entry(reg).or_default(),
            Value::Integer(ival) => ival,
        };
        if self.eip + offset >= 0 {
            if let Some(to_modify) = self.prog.get(usize::try_from(self.eip + offset).unwrap()) {
                let new = match *to_modify {
                    Instructions::Cpy(x, y) => Instructions::Jnz(x, y),
                    Instructions::Inc(x) => Instructions::Dec(x),
                    Instructions::Tgl(x) | Instructions::Dec(x) => Instructions::Inc(x),
                    Instructions::Jnz(x, y) => Instructions::Cpy(x, y),
                    Instructions::Out(_) => panic!(),
                };
                self.prog[usize::try_from(self.eip + offset).unwrap()] = new;
            };
        }
        self.eip += 1;
    }

    fn out(&mut self, value: Value) -> bool {
        let value = match value {
            Value::Register(reg) => *self.register.entry(reg).or_default(),
            Value::Integer(ival) => ival,
        };
        self.output.push(value);
        // Arbitrary value!
        if self.output.len() == 500 {
            return false;
        }
        self.eip += 1;
        self.output
            .chunks_exact(2)
            .all(|chunk| chunk[0] == 0 && chunk[1] == 1)
    }

    fn run_instrcution(&mut self, instr: &Instructions) {
        match instr {
            Instructions::Cpy(x, Value::Register(y)) => self.cpy(*x, *y),
            Instructions::Inc(Value::Register(reg)) => self.inc(*reg),
            Instructions::Dec(Value::Register(reg)) => self.dec(*reg),
            Instructions::Jnz(x, offset) => self.jnz(*x, *offset),
            Instructions::Tgl(x) => self.toggle(*x),
            Instructions::Out(x) => {
                if !self.out(*x) {
                    self.eip = isize::MAX;
                }
            }
            _ => {
                self.eip += 1;
            }
        }
    }

    fn run_prog(&mut self) {
        while let Some(&instr) = self
            .prog
            .get(usize::try_from(self.eip).unwrap_or(usize::MAX))
        {
            self.run_instrcution(&instr);
        }
    }
}

fn main() {
    let input = include_str!("../data/day_2016_25.data")
        .lines()
        .map(str::parse::<Instructions>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    for idx in 0.. {
        if let Some(v) = execute(&input, idx) {
            println!("Part 1: {}", v);
            break;
        }
    }
}

fn execute(input: &[Instructions], value_a: isize) -> Option<isize> {
    //  println!("Executing: {}", value_a);
    let mut cpu = Cpu::new(input);
    cpu.set_register('a', value_a);
    cpu.run_prog();
    if cpu.output.starts_with(&[0, 1, 0, 1, 0, 1, 0, 1, 0, 1]) {
        Some(value_a)
    } else {
        None
    }
}
