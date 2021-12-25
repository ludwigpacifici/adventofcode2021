use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    let positions = parse(&input);

    println!("Part 1: {}", part1(positions.clone()));
    println!("Part 2: {}", part2(&positions));
}

fn part1(mut positions: Vec<usize>) -> usize {
    let n = positions.len();
    positions.sort_unstable();
    let median = if n % 2 == 0 {
        positions[(n + 1) / 2]
    } else {
        (positions[n / 2] + positions[(n + 1) / 2]) / 2
    };

    positions
        .into_iter()
        .map(|n| if n > median { n - median } else { median - n })
        .sum()
}

fn part2(positions: &[usize]) -> usize {
    let sum: usize = positions.iter().sum();
    let mean_floor = sum / positions.len();
    let mean_ceil = mean_floor + 1;

    cost2(positions, mean_floor).min(cost2(positions, mean_ceil))
}

fn cost2(positions: &[usize], target: usize) -> usize {
    positions
        .iter()
        .map(|&n| {
            let d = if n > target { n - target } else { target - n };
            (d * (d + 1)) / 2
        })
        .sum()
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().expect("Cannot parse input integer"))
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(parse("16,1,2,0,4,2,7,1,2,14")), 37);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&parse("16,1,2,0,4,2,7,1,2,14")), 168);
    }
}
