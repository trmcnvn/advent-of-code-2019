use itertools::Itertools;
use std::collections::VecDeque;

#[derive(PartialEq)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equal,
    Halt,
}
impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpTrue,
            6 => Opcode::JumpFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equal,
            99 => Opcode::Halt,
            _ => unreachable!(),
        }
    }
}

enum ParamMode {
    Immediate,
    Position,
}

#[derive(Debug)]
struct Program {
    input: VecDeque<usize>,
    instructions: Vec<isize>,
    last_diag_code: Option<usize>,
    cursor: usize,
}

impl Program {
    fn new(instructions: &[isize], sequence: &[usize]) -> Self {
        Self {
            input: VecDeque::from(sequence.to_vec()),
            instructions: instructions.to_vec(),
            last_diag_code: None,
            cursor: 0,
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
                    self.insert(1, input as isize);
                    self.cursor += 2;
                }
            }
            Opcode::Output => {
                let addr = self.instructions[self.cursor + 1];
                self.last_diag_code = Some(self.instructions[addr as usize] as usize);
                self.cursor += 2;
            }
            Opcode::JumpTrue => {
                if self.param(1) > 0 {
                    self.cursor = self.param(2) as usize;
                } else {
                    self.cursor += 3;
                }
            }
            Opcode::JumpFalse => {
                if self.param(1) == 0 {
                    self.cursor = self.param(2) as usize;
                } else {
                    self.cursor += 3;
                }
            }
            Opcode::LessThan => {
                self.insert(3, (self.param(1) < self.param(2)).into());
                self.cursor += 4;
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
        (self.instructions[self.cursor] as usize)
            .rem_euclid(100)
            .into()
    }

    fn param(&self, offset: usize) -> isize {
        let value = self.instructions[self.cursor + offset];
        match self.param_mode(offset) {
            ParamMode::Immediate => value,
            ParamMode::Position => self.instructions[value as usize],
        }
    }

    fn insert(&mut self, offset: usize, value: isize) {
        let addr = self.instructions[self.cursor + offset];
        self.instructions[addr as usize] = value;
    }

    fn add_input(&mut self, input: usize) {
        self.input.push_back(input);
    }

    fn param_mode(&self, offset: usize) -> ParamMode {
        let opcode = self.instructions[self.cursor] as usize;
        if (opcode / 10usize.pow(offset as u32 + 1)).rem_euclid(10) == 1 {
            ParamMode::Immediate
        } else {
            ParamMode::Position
        }
    }

    fn diag_code(&self) -> Option<usize> {
        self.last_diag_code
    }
}

fn part1(input: &str) -> usize {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<isize>().ok())
        .collect();
    (0..=4)
        .permutations(5)
        .map(|sequence| {
            let mut last = 0;
            sequence.iter().for_each(|v| {
                let mut program = Program::new(&input, &[*v, last]);
                program.run();
                last = program.diag_code().unwrap_or(0);
            });
            last
        })
        .max()
        .unwrap_or(0)
}

fn part2(input: &str) -> usize {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<isize>().ok())
        .collect();
    let mut bests = Vec::new();
    for sequence in (5..=9).permutations(5) {
        let mut programs = Vec::new();
        for value in sequence.iter() {
            programs.push(Program::new(&input, &[*value]));
        }
        let mut last_value = 0;
        'feedback: loop {
            for (idx, program) in programs.iter_mut().enumerate() {
                program.add_input(last_value);
                loop {
                    match program.step() {
                        Opcode::Output => {
                            last_value = program.diag_code().unwrap_or(0);
                            break;
                        }
                        Opcode::Halt if idx == 4 => break 'feedback,
                        Opcode::Halt => break,
                        _ => continue,
                    }
                }
            }
        }
        bests.push(last_value);
    }
    *bests.iter().max().unwrap_or(&0)
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
        assert_eq!(
            43210,
            part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
        );
        assert_eq!(
            54321,
            part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
        );
        assert_eq!(65210, part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            18216,
            part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")
        );
    }
}
