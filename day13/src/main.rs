use std::cell::Cell;

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

enum ParamMode {
    Immediate,
    Position,
    Relative,
}

struct Program<F>
where
    F: Fn() -> isize,
{
    instructions: Vec<isize>,
    last_diag_code: isize,
    cursor: isize,
    relative_base: isize,
    input_func: F,
}

impl<F> Program<F>
where
    F: Fn() -> isize,
{
    fn new(instructions: &[isize], func: F) -> Self {
        let mut memory = instructions.to_vec();
        memory.resize(instructions.len() * 10, 0);
        Self {
            instructions: memory,
            last_diag_code: 0,
            cursor: 0,
            relative_base: 0,
            input_func: func,
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
                self.insert(1, (self.input_func)());
                self.cursor += 2;
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

fn part1(input: &str) -> usize {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<isize>().ok())
        .collect();
    let mut program = Program::new(&input, || 0);
    let mut outputs = Vec::new();
    loop {
        match program.step() {
            Opcode::Output => outputs.push(program.diag_code()),
            Opcode::Halt => break,
            _ => continue,
        }
    }
    outputs.chunks(3).filter(|c| c[2] == 2).count()
}

fn part2(input: &str) -> usize {
    let mut input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<isize>().ok())
        .collect();
    input[0] = 2;

    let mut outputs = Vec::new();
    let mut score = 0;
    let paddle_x = Cell::new(0);
    let ball_x = Cell::new(0);

    let mut program = Program::new(&input, || {
        ((ball_x.get() - paddle_x.get()) as isize).signum()
    });

    loop {
        match program.step() {
            Opcode::Output => {
                if outputs.len() < 2 {
                    outputs.push(program.diag_code())
                } else {
                    match program.diag_code() {
                        3 => paddle_x.set(outputs[0]),
                        4 => ball_x.set(outputs[0]),
                        n if outputs[0] == -1 => score = n,
                        _ => {}
                    };
                    outputs.clear();
                }
            }
            Opcode::Halt => break,
            _ => continue,
        }
    }
    score as usize
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
