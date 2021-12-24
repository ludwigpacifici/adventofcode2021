use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    let model = parse(&input);

    println!("Part 1: {}", part1(model.clone()));
    println!("Part 2: {}", part2(model));
}

type Model = [usize; 9];

fn part1(model: Model) -> usize {
    solve(model, 80)
}

fn part2(model: Model) -> usize {
    solve(model, 256)
}

fn solve(model: Model, days: usize) -> usize {
    (0..days).fold(model, |m, _| iteration(m)).into_iter().sum()
}

fn iteration(m: Model) -> Model {
    [m[1], m[2], m[3], m[4], m[5], m[6], m[7] + m[0], m[8], m[0]]
}

fn parse(input: &str) -> Model {
    input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().expect("Cannot parse number input"))
        .fold([0usize; 9], |mut acc, n| {
            acc[n] += 1;
            acc
        })
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(parse("3,4,3,1,2")), 5934);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(parse("3,4,3,1,2")), 26984457539);
    }
}
