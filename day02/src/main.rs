use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");
    let input = input
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            let direction = it.next().expect("Cannot read direction");
            let unit = it
                .next()
                .expect("Cannot read unit")
                .parse::<u64>()
                .expect("Cannot parse unit as integer");
            (direction, unit)
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[(&str, u64)]) -> u64 {
    let (horizontal, depth) = input.iter().fold(
        (0, 0),
        |(horizontal, depth), (direction, unit)| match *direction {
            "forward" => (horizontal + unit, depth),
            "down" => (horizontal, depth + unit),
            "up" => (horizontal, depth - unit),
            d => panic!("Unknown direction {}", d),
        },
    );

    horizontal * depth
}

fn part2(input: &[(&str, u64)]) -> u64 {
    let (horizontal, depth, _aim) = input.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), (direction, unit)| match *direction {
            "forward" => (horizontal + unit, depth + aim * unit, aim),
            "down" => (horizontal, depth, aim + unit),
            "up" => (horizontal, depth, aim - unit),
            d => panic!("Unknown direction {}", d),
        },
    );

    horizontal * depth
}
