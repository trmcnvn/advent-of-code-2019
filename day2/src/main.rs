fn run_program(input: &[usize], noun: usize, verb: usize) -> Vec<usize> {
    let mut program = input.to_vec();
    program[1] = noun;
    program[2] = verb;

    let mut ptr = 0;
    loop {
        let snapshot = program.clone();
        match program[ptr] {
            1 => {
                let params = &snapshot[ptr + 1..=ptr + 3];
                program[params[2]] = program[params[0]] + program[params[1]];
            }
            2 => {
                let params = &snapshot[ptr + 1..=ptr + 3];
                program[params[2]] = program[params[0]] * program[params[1]];
            }
            99 => break,
            _ => unreachable!(),
        };
        ptr += 4;
    }

    program
}

fn part1(input: &str) -> usize {
    let input: Vec<usize> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<usize>().ok())
        .collect();
    run_program(&input, 12, 2)[0]
}

fn part2(input: &str) -> usize {
    let input: Vec<usize> = input
        .trim()
        .split(',')
        .flat_map(|v| v.parse::<usize>().ok())
        .collect();

    let search: Vec<usize> = (0..(99usize.pow(2))).collect();
    search
        .binary_search_by(|probe| {
            let result = run_program(&input, probe.div_euclid(100), probe.rem_euclid(100))[0];
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
            run_program(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 9, 10)[0]
        );
        assert_eq!(2, run_program(&[1, 0, 0, 0, 99], 0, 0)[0]);
        assert_eq!(2, run_program(&[2, 3, 0, 3, 99], 3, 0)[0]);
        assert_eq!(2, run_program(&[2, 4, 4, 5, 99, 0], 4, 4)[0]);
        assert_eq!(30, run_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99], 1, 1)[0]);
    }
}
