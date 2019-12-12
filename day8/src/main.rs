use std::collections::{BTreeMap, HashMap};

fn part1(input: &str, width: usize, height: usize) -> usize {
    let input = input.trim().chars().collect::<Vec<_>>();
    let input = input.chunks(width * height).collect::<Vec<_>>();
    let mut image = HashMap::new();
    let mut zeros_count = (0, 0);
    for (idx, layer) in input.iter().enumerate() {
        let count = layer.iter().filter(|c| **c != '0').count();
        if count > zeros_count.1 {
            zeros_count = (idx, count);
        }
        image.insert(idx, layer);
    }
    image[&zeros_count.0].iter().filter(|c| **c == '1').count()
        * image[&zeros_count.0].iter().filter(|c| **c == '2').count()
}

fn part2(input: &str, width: usize, height: usize) -> String {
    let input = input.trim().chars().collect::<Vec<_>>();
    let input = input.chunks(width * height).collect::<Vec<_>>();

    let mut thing = BTreeMap::new();
    for layer in input {
        for (idx, pixel) in layer.iter().enumerate() {
            match pixel {
                '0' | '1' => thing.entry(idx).or_insert(pixel),
                _ => continue,
            };
        }
    }

    for a in thing.values().collect::<Vec<_>>().chunks(25) {
        for b in a {
            print!("{}", if ***b == '0' { '.' } else { '#' });
        }
        println!();
    }
    thing.values().map(|c| **c).collect()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input, 25, 6));
    println!("Part 2: {}", part2(input, 25, 6));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("0110", part2("0222112222120000", 2, 2));
    }
}
