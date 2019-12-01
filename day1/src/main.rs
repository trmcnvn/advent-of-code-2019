fn part1(input: &str) {
    let mut fuel_vec: Vec<u32> = Vec::new();
    for line in input.lines() {
        let value = line.parse::<u32>().unwrap();
        let fuel = (value / 3) - 2;
        fuel_vec.push(fuel);
    }
    let sum: u32 = fuel_vec.iter().sum();
    println!("Part1: {}", sum);
}

fn part2(input: &str) {
    let mut fuel_vec: Vec<i32> = Vec::new();
    for line in input.lines() {
        let fuel_fn = |value: i32| -> i32 { (value / 3) - 2 };
        let value = line.parse::<i32>().unwrap();
        let mut fuel = fuel_fn(value);
        while fuel > 0 {
            fuel_vec.push(fuel);
            fuel = fuel_fn(fuel);
        }
    }
    let sum: i32 = fuel_vec.iter().sum();
    println!("Part2: {}", sum);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}
