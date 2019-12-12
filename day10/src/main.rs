use std::collections::HashSet;

type Point = (isize, isize);

fn parse_input(input: &str) -> HashSet<Point> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect()
}

fn gcd(x: isize, y: isize) -> isize {
    if x == 0 {
        y.abs()
    } else {
        gcd(y % x, x)
    }
}

fn reduced_gcd(a: &Point, b: &Point) -> Point {
    let delta_x = b.0 - a.0;
    let delta_y = b.1 - a.1;
    let gcd = gcd(delta_x, delta_y);
    (delta_x / gcd, delta_y / gcd)
}

fn find_in_los(station: &Point, asteroids: &HashSet<Point>) -> HashSet<Point> {
    let mut targets = HashSet::new();
    for asteroid in asteroids.iter().filter(|a| *a != station) {
        targets.insert(reduced_gcd(station, asteroid));
    }
    targets
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);
    map.iter()
        .map(|a| find_in_los(a, &map).len())
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let targets: Vec<_> = map.iter().map(|a| (a, find_in_los(a, &map))).collect();
    let (station, mut targets) = targets
        .iter()
        .map(|(station, points)| (station, points.iter().collect::<Vec<_>>()))
        .max_by_key(|(_, points)| points.len())
        .unwrap();
    targets.sort_unstable_by(|a, b| {
        let a = (a.0 as f32).atan2(a.1 as f32);
        let b = (b.0 as f32).atan2(b.1 as f32);
        b.partial_cmp(&a).unwrap()
    });
    let target = &targets[199];
    let mut x = station.0 + target.0;
    let mut y = station.1 + target.1;
    while !map.contains(&(x, y)) {
        x += target.0;
        y += target.1;
    }
    (x * 100 + y) as usize
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
        assert_eq!(
            33,
            part1(
                r"
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            )
        );
    }
}
