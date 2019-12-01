fn part1(input: &str) {
    let sum: u32 = input
        .lines()
        .map(|line| (line.parse::<u32>().unwrap() / 3) - 2)
        .sum();
    println!("Part1: {}", sum);
}

fn part2(input: &str) {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let calc_fn = |value: i32| -> i32 { (value / 3) - 2 };
            let mut base = calc_fn(line.parse::<i32>().unwrap());
            let mut total = 0;
            while base > 0 {
                total += base;
                base = calc_fn(base);
            }
            total
        })
        .sum();
    println!("Part2: {}", sum);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
