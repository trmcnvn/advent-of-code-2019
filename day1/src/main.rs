fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| l.parse::<usize>().ok())
        .map(|d| (d / 3) - 2)
        .sum()
}

fn part2(input: &str) -> usize {
    fn calc_fn(d: usize) -> usize {
        d.checked_div(3)
            .and_then(|d| d.checked_sub(2))
            .map(|d| d + calc_fn(d))
            .unwrap_or(0)
    };
    input
        .lines()
        .flat_map(|l| l.parse::<usize>().ok())
        .map(calc_fn)
        .sum()
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
    fn test_part1() {
        assert_eq!(2, part1("12"));
        assert_eq!(2, part1("14"));
        assert_eq!(654, part1("1969"));
        assert_eq!(33583, part1("100756"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2("14"));
        assert_eq!(966, part2("1969"));
        assert_eq!(50346, part2("100756"));
    }
}
