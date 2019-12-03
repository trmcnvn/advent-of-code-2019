use std::collections::HashMap;

type Positions = HashMap<(isize, isize), usize>;
fn parse_actions(actions: &[char]) -> Positions {
    let mut positions = Positions::new();
    let mut xys = (0isize, 0isize, 0usize);
    for action in actions {
        match action {
            'R' => xys.0 += 1,
            'L' => xys.0 -= 1,
            'U' => xys.1 -= 1,
            'D' => xys.1 += 1,
            _ => unreachable!(),
        };
        xys.2 += 1;
        positions.insert((xys.0, xys.1), xys.2);
    }
    positions
}

fn part1(input: &str) -> usize {
    let wires: Vec<_> = input
        .lines()
        .map(|w| w.split(','))
        .map(|w| {
            w.flat_map(|a| {
                let direction = a.chars().next().unwrap();
                let steps = a[1..].parse::<usize>().unwrap();
                vec![direction; steps]
            })
            .collect::<Vec<_>>()
        })
        .map(|w| parse_actions(&w))
        .collect();
    wires[0]
        .keys()
        .filter(|k| wires[1].contains_key(&k))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap() as usize
}

fn part2(input: &str) -> usize {
    let wires: Vec<_> = input
        .lines()
        .map(|w| w.split(','))
        .map(|w| {
            w.flat_map(|a| {
                let direction = a.chars().next().unwrap();
                let steps = a[1..].parse::<usize>().unwrap();
                vec![direction; steps]
            })
            .collect::<Vec<_>>()
        })
        .map(|w| parse_actions(&w))
        .collect();
    wires[0]
        .keys()
        .filter(|k| wires[1].contains_key(&k))
        .map(|k| wires[0][&k] + wires[1][&k])
        .min()
        .unwrap() as usize
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
        assert_eq!(6, part1("R8,U5,L5,D3\nU7,R6,D4,L4"));
        assert_eq!(
            159,
            part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
        );
        assert_eq!(
            135,
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2("R8,U5,L5,D3\nU7,R6,D4,L4"));
        assert_eq!(
            610,
            part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
        );
        assert_eq!(
            410,
            part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }
}
