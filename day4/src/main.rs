fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split('-')
        .flat_map(|s| s.parse::<usize>().ok())
        .collect()
}

fn part1(input: &str) -> usize {
    let ranges = parse_input(input);
    let mut count = 0;
    for password in ranges[0]..=ranges[1] {
        let digits = password.to_string().chars().collect::<Vec<_>>();
        let sequencing = digits.windows(2).any(|p| p[0] == p[1]);
        let ascending = digits.windows(2).all(|p| p[0] <= p[1]);
        if sequencing && ascending {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let ranges: Vec<_> = parse_input(input);
    let mut count = 0;
    for password in ranges[0]..=ranges[1] {
        let mut digits = password.to_string();
        let mut last_digit = digits.pop().unwrap();
        let mut sequence_count = 1;
        let mut has_double = false;
        let mut is_invalid = false;
        while !digits.is_empty() {
            let current_digit = digits.pop().unwrap();
            if current_digit < last_digit {
                if sequence_count == 2 {
                    has_double = true;
                }
                sequence_count = 1;
            } else if current_digit == last_digit {
                sequence_count += 1;
            } else if current_digit > last_digit {
                is_invalid = true;
                break;
            }
            last_digit = current_digit;
        }
        if !is_invalid && (has_double || sequence_count == 2) {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
