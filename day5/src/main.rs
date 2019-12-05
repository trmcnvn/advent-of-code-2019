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

struct Program {
    input: usize,
    instructions: Vec<isize>,
    last_diag_code: isize,
    cursor: usize,
}

impl Program {
    fn new(instructions: &[isize], input: usize) -> Self {
        Self {
            input,
            instructions: instructions.to_vec(),
            last_diag_code: 0,
            cursor: 0,
        }
    }

    fn run(&mut self) {
        loop {
            let opcode: Opcode = self.opcode().rem_euclid(100).into();
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
                    self.insert(1, self.input as isize);
                    self.cursor += 2;
                }
                Opcode::Output => {
                    let addr = self.instructions[self.cursor + 1];
                    self.last_diag_code = self.instructions[addr as usize];
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
                Opcode::Halt => break,
            };
        }
    }

    fn opcode(&self) -> usize {
        self.instructions[self.cursor] as usize
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

    fn param_mode(&self, offset: usize) -> ParamMode {
        if (self.opcode() / 10usize.pow(offset as u32 + 1)).rem_euclid(10) == 1 {
            ParamMode::Immediate
        } else {
            ParamMode::Position
        }
    }

    fn diag_code(&self) -> usize {
        self.last_diag_code as usize
    }
}

fn run(input: &str, program_input: usize) -> usize {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<isize>().ok())
        .collect();
    let mut program = Program::new(&input, program_input);
    program.run();
    program.diag_code()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", run(input, 1));
    println!("Part 2: {}", run(input, 5));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program() {
        assert_eq!(0, run("1002,4,3,4,33", 1));
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(1000, run(input, 8));
    }
}
