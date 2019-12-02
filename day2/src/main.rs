fn part1(input: &str) {
    let mut integers = input
        .replace("\r\n", "")
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    integers[1] = 12;
    integers[2] = 2;

    let mut index = 0;
    loop {
        let a = integers[index + 1];
        let b = integers[index + 2];
        let c = integers[index + 3];
        match integers[index] {
            1 => {
                integers[c] = integers[a] + integers[b];
            }
            2 => {
                integers[c] = integers[a] * integers[b];
            }
            99 => break,
            _ => unreachable!(),
        };
        index += 4;
    }

    println!("Part 1: {}", integers[0]);
}

fn part2(input: &str) {
    let integers = input
        .replace("\r\n", "")
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = integers.clone();
            memory[1] = noun;
            memory[2] = verb;

            let mut index = 0;
            loop {
                let a = memory[index + 1];
                let b = memory[index + 2];
                let c = memory[index + 3];
                match memory[index] {
                    1 => {
                        memory[c] = memory[a] + memory[b];
                    }
                    2 => {
                        memory[c] = memory[a] * memory[b];
                    }
                    99 => break,
                    _ => unreachable!(),
                };
                index += 4;
            }

            if memory[0] == 19_690_720 {
                println!("Part 2: {}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
