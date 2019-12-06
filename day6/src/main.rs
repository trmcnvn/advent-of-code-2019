use std::collections::HashMap;

#[derive(Debug)]
struct OrbitMap {
    pub map: HashMap<&'static str, Option<&'static str>>,
}

impl OrbitMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, object: &'static str, link: &'static str) {
        self.map.entry(object).or_insert(None);
        let child = self.map.entry(link).or_insert(None);
        *child = Some(object);
    }

    fn walk(&self, object: Option<&'static str>) -> Vec<&str> {
        let mut result = Vec::new();
        let mut parent = object;
        while parent.is_some() {
            result.push(parent.unwrap());
            let object = *self.map.get(parent.unwrap()).unwrap();
            parent = object;
        }
        result
    }
}

fn parse_input(input: &'static str) -> OrbitMap {
    let mut orbit = OrbitMap::new();
    input
        .lines()
        .map(|l| l.split(')').collect::<Vec<_>>())
        .for_each(|o| orbit.insert(o[0], o[1]));
    orbit
}

fn part1(input: &'static str) -> usize {
    let orbit = parse_input(input);
    let mut count = 0;
    for value in orbit.map.values() {
        let parents = orbit.walk(*value);
        count += parents.len();
    }
    count
}

fn part2(input: &'static str) -> usize {
    let orbit = parse_input(input);
    let you = orbit.map.get("YOU").unwrap();
    let san = orbit.map.get("SAN").unwrap();
    let you_parents = orbit.walk(*you);
    let san_parents = orbit.walk(*san);

    for (a, b) in you_parents.iter().enumerate() {
        for (c, d) in san_parents.iter().enumerate() {
            if b == d {
                return a + c;
            }
        }
    }
    unreachable!()
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
            42,
            part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            4,
            part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN")
        );
    }
}
