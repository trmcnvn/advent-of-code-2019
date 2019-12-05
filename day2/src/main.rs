enum Opcode {
    Add,
    Mul,
    Halt,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            99 => Opcode::Halt,
            _ => unreachable!(),
        }
    }
}

struct Program {
    instructions: Vec<usize>,
    cursor: usize,
}

impl Program {
    fn new(instructions: &[usize], input: (usize, usize)) -> Self {
        let mut p = Self {
            instructions: instructions.to_vec(),
            cursor: 0,
        };
        p.set_input(input);
        p
    }

    fn set_input(&mut self, input: (usize, usize)) {
        self.instructions[1] = input.0;
        self.instructions[2] = input.1;
    }

    fn run(&mut self) -> &mut Self {
        loop {
            let opcode: Opcode = self.opcode().into();
            match opcode {
                Opcode::Add => {
                    self.insert(3, self.param(1) + self.param(2));
                    self.cursor += 4;
                }
                Opcode::Mul => {
                    self.insert(3, self.param(1) * self.param(2));
                    self.cursor += 4;
                }
                Opcode::Halt => break,
            };
        }
        self
    }

    fn opcode(&self) -> usize {
        self.instructions[self.cursor]
    }

    fn param(&self, offset: usize) -> usize {
        let value = self.instructions[self.cursor + offset];
        self.instructions[value]
    }

    fn insert(&mut self, offset: usize, value: usize) {
        let addr = self.instructions[self.cursor + offset];
        self.instructions[addr] = value;
    }

    fn result(&self) -> usize {
        self.instructions[0]
    }
}

fn part1(input: &str) -> usize {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<usize>().ok())
        .collect();
    let mut program = Program::new(&input, (12, 2));
    program.run();
    program.result()
}

fn part2(input: &str) -> usize {
    let input: Vec<_> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<usize>().ok())
        .collect();
    let search: Vec<_> = (0..(99usize.pow(2))).collect();
    search
        .binary_search_by(|probe| {
            let mut program = Program::new(&input, (probe.div_euclid(100), probe.rem_euclid(100)));
            program.run();
            let result = program.result();
            result.cmp(&19_690_720)
        })
        .unwrap_or(0)
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
    fn test_program() {
        assert_eq!(
            3500,
            Program::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], (9, 10))
                .run()
                .result()
        );
        assert_eq!(2, Program::new(&[1, 0, 0, 0, 99], (0, 0)).run().result());
    }
}
