use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32, i32);

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Point(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    id: usize,
    position: Point,
    velocity: Point,
}

fn parse_input(input: &str) -> Vec<Moon> {
    input
        .lines()
        .flat_map(|l| scan_fmt!(l, "<x={d}, y={d}, z={d}>", i32, i32, i32).ok())
        .enumerate()
        .map(|(id, p)| Moon {
            id,
            position: Point(p.0, p.1, p.2),
            velocity: Point(0, 0, 0),
        })
        .collect()
}

fn velocity_change(a: i32, b: i32) -> i32 {
    use std::cmp::Ordering;
    match a.cmp(&b) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

fn gcd(x: isize, y: isize) -> isize {
    if x == 0 {
        y.abs()
    } else {
        gcd(y % x, x)
    }
}

fn lcm(x: isize, y: isize) -> isize {
    x * y / gcd(x, y)
}

fn part1(input: &str, steps: usize) -> usize {
    let mut moons = parse_input(input);
    for _ in 0..steps {
        let fake_moons_because_im_tired = moons.clone();
        fake_moons_because_im_tired
            .iter()
            .permutations(2)
            .for_each(|pairs| {
                moons[pairs[0].id].velocity += Point(
                    velocity_change(pairs[0].position.0, pairs[1].position.0),
                    velocity_change(pairs[0].position.1, pairs[1].position.1),
                    velocity_change(pairs[0].position.2, pairs[1].position.2),
                );
            });
        for moon in &mut moons {
            moon.position += moon.velocity;
        }
    }
    moons
        .iter()
        .map(|moon| {
            let pot = moon.position.0.abs() + moon.position.1.abs() + moon.position.2.abs();
            let kin = moon.velocity.0.abs() + moon.velocity.1.abs() + moon.velocity.2.abs();
            (pot * kin) as usize
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut moons = parse_input(input);
    let mut num_matches = 0;
    let mut idx = 0;
    let mut data = (0, 0, 0);
    while num_matches < 3 {
        idx += 1;
        let fake_moons_because_im_tired = moons.clone();
        fake_moons_because_im_tired
            .iter()
            .permutations(2)
            .for_each(|pairs| {
                moons[pairs[0].id].velocity += Point(
                    velocity_change(pairs[0].position.0, pairs[1].position.0),
                    velocity_change(pairs[0].position.1, pairs[1].position.1),
                    velocity_change(pairs[0].position.2, pairs[1].position.2),
                );
            });
        for moon in &mut moons {
            moon.position += moon.velocity;
        }
        if data.0 == 0 && moons.iter().all(|m| m.velocity.0 == 0) {
            num_matches += 1;
            data.0 = idx * 2;
        }
        if data.1 == 0 && moons.iter().all(|m| m.velocity.1 == 0) {
            num_matches += 1;
            data.1 = idx * 2;
        }
        if data.2 == 0 && moons.iter().all(|m| m.velocity.2 == 0) {
            num_matches += 1;
            data.2 = idx * 2;
        }
    }
    lcm(lcm(data.0, data.1), data.2) as usize
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input, 1000));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            1940,
            part1(
                "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
                100
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            4_686_774_924,
            part2("<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>")
        );
    }
}
