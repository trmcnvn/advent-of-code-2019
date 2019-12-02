fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

fn run_program(integers: &mut [usize]) {
    let mut idx = 0;
    loop {
        let params = integers.get(idx + 1..=idx + 3).unwrap();
        match integers[idx] {
            1 => {
                integers[params[2]] = integers[params[0]] + integers[params[1]];
            }
            2 => {
                integers[params[2]] = integers[params[0]] * integers[params[1]];
            }
            99 => break,
            _ => unreachable!(),
        };
        idx += 4;
    }
}

fn part1(input: &str) {
    let mut integers = parse_input(input);
    integers[1] = 12;
    integers[2] = 2;
    run_program(&mut integers);
    println!("Part 1: {}", integers[0]);
}

fn part2(input: &str) {
    let integers = parse_input(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = integers.clone();
            memory[1] = noun;
            memory[2] = verb;
            run_program(&mut memory);
            if memory[0] == 19_690_720 {
                return println!("Part 2: {}", 100 * noun + verb);
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
