use std::collections::{BTreeMap, VecDeque};

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

    fn add_input(&mut self, input: isize) {
        self.input.push_back(input);
    }

    fn diag_code(&self) -> isize {
        self.last_diag_code
    }
}

#[derive(PartialEq)]
enum Panel {
    Black,
    White,
}
impl From<isize> for Panel {
    fn from(value: isize) -> Self {
        match value {
            0 => Panel::Black,
            1 => Panel::White,
            _ => unreachable!(),
        }
    }
}
impl From<&mut Panel> for isize {
    fn from(value: &mut Panel) -> isize {
        match value {
            Panel::Black => 0,
            Panel::White => 1,
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl From<isize> for Direction {
    fn from(value: isize) -> Self {
        match value {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => unreachable!(),
        }
    }
}

fn part1(input: &str, starting_color: isize) -> BTreeMap<(i32, i32), Panel> {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<isize>().ok())
        .collect();

    let mut robot = BTreeMap::new();
    let mut program = Program::new(&input, &[starting_color]);
    let mut position = (0, 0);
    robot.insert(position, Panel::Black);

    let mut is_color_output = true;
    let mut facing_direction = Direction::Up;
    loop {
        match program.step() {
            Opcode::Output => {
                if is_color_output {
                    let color: Panel = program.diag_code().into();
                    *robot.entry(position).or_insert(Panel::Black) = color;
                    is_color_output = false;
                } else {
                    facing_direction = match program.diag_code().into() {
                        Direction::Left => match facing_direction {
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                        },
                        Direction::Right => match facing_direction {
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                        },
                        _ => unreachable!(),
                    };
                    position = match facing_direction {
                        Direction::Left => (position.0 - 1, position.1),
                        Direction::Right => (position.0 + 1, position.1),
                        Direction::Up => (position.0, position.1 - 1),
                        Direction::Down => (position.0, position.1 + 1),
                    };
                    program.add_input(robot.entry(position).or_insert(Panel::Black).into());
                    is_color_output = true;
                }
            }
            Opcode::Halt => break,
            _ => continue,
        }
    }
    robot
}

fn part2(input: &str) {
    let map = part1(input, 1);
    let max_y = map.keys().max_by_key(|(_, y)| y).unwrap().1;
    let max_x = map.keys().max_by_key(|(x, _)| x).unwrap().0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if map.contains_key(&(x, y)) {
                print!(
                    "{}",
                    if map[&(x, y)] == Panel::White {
                        '#'
                    } else {
                        '.'
                    }
                );
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input, 0).keys().len());
    part2(input);
}
