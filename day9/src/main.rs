use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equal,
    RelativeBase,
    Halt,
}
impl From<isize> for Opcode {
    fn from(value: isize) -> Self {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpTrue,
            6 => Opcode::JumpFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equal,
            9 => Opcode::RelativeBase,
            99 => Opcode::Halt,
            val => unreachable!(val),
        }
    }
}

#[derive(Debug)]
enum ParamMode {
    Immediate,
    Position,
    Relative,
}

#[derive(Debug)]
struct Program {
    input: VecDeque<isize>,
    instructions: Vec<isize>,
    last_diag_code: isize,
    cursor: isize,
    relative_base: isize,
}

impl Program {
    fn new(instructions: &[isize], sequence: &[isize]) -> Self {
        let mut memory = instructions.to_vec();
        memory.resize(instructions.len() * 10, 0);
        Self {
            input: VecDeque::from(sequence.to_vec()),
            instructions: memory,
            last_diag_code: 0,
            cursor: 0,
            relative_base: 0,
        }
    }

    fn step(&mut self) -> Opcode {
        let opcode = self.opcode();
        match opcode {
            Opcode::Add => {
                self.insert(3, self.param(1) + self.param(2));
                self.cursor += 4;
            }
            Opcode::Mul => {
                self.insert(3, self.param(1) * self.param(2));
                self.cursor += 4;
            }
            Opcode::Input => {
                if let Some(input) = self.input.pop_front() {
                    self.insert(1, input);
                    self.cursor += 2;
                }
            }
            Opcode::Output => {
                self.last_diag_code = self.param(1);
                self.cursor += 2;
            }
            Opcode::JumpTrue => {
                if self.param(1) > 0 {
                    self.cursor = self.param(2);
                } else {
                    self.cursor += 3;
                }
            }
            Opcode::JumpFalse => {
                if self.param(1) == 0 {
                    self.cursor = self.param(2);
                } else {
                    self.cursor += 3;
                }
            }
            Opcode::LessThan => {
                self.insert(3, (self.param(1) < self.param(2)).into());
                self.cursor += 4;
            }
            Opcode::RelativeBase => {
                self.relative_base += self.param(1);
                self.cursor += 2;
            }
            Opcode::Equal => {
                self.insert(3, (self.param(1) == self.param(2)).into());
                self.cursor += 4;
            }
            _ => {}
        };
        opcode
    }

    fn run(&mut self) -> bool {
        loop {
            if self.step() == Opcode::Halt {
                return true;
            }
        }
    }

    fn opcode(&self) -> Opcode {
        (self.instructions[self.cursor as usize])
            .rem_euclid(100)
            .into()
    }

    fn param(&self, offset: isize) -> isize {
        let value = self.instructions[(self.cursor + offset) as usize];
        match self.param_mode(offset) {
            ParamMode::Immediate => value,
            ParamMode::Position => self.instructions[value as usize],
            ParamMode::Relative => self.instructions[(self.relative_base + value) as usize],
        }
    }

    fn insert(&mut self, offset: isize, value: isize) {
        let position = match self.param_mode(offset) {
            ParamMode::Position => self.instructions[(self.cursor + offset) as usize],
            ParamMode::Relative => {
                self.relative_base + self.instructions[(self.cursor + offset) as usize]
            }
            _ => unreachable!(),
        };
        self.instructions[position as usize] = value;
    }

    fn param_mode(&self, offset: isize) -> ParamMode {
        let opcode = self.instructions[self.cursor as usize];
        match (opcode / 10isize.pow(offset as u32 + 1)).rem_euclid(10) {
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => ParamMode::Position,
        }
    }

    fn diag_code(&self) -> isize {
        self.last_diag_code
    }
}

fn part1(input: &str) -> isize {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<isize>().ok())
        .collect();
    let mut program = Program::new(&input, &[1]);
    program.run();
    program.diag_code()
}

fn part2(input: &str) -> isize {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<isize>().ok())
        .collect();
    let mut program = Program::new(&input, &[2]);
    program.run();
    program.diag_code()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let digit = part1("1102,34915192,34915192,7,4,7,99,0");
        assert_eq!(16, digit.to_string().len());
        assert_eq!(1_125_899_906_842_624, part1("104,1125899906842624,99"));
    }
}
