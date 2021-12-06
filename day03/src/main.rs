use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");
    let numbers = parse(&input);

    println!("Part 1: {}", part1(numbers.clone()));
    println!("Part 2: {}", part2(numbers));
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '1' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part1(numbers: Vec<Vec<u64>>) -> u64 {
    let mut most_significants = Vec::new();
    let mut least_significants = Vec::new();
    for column in 0..numbers[0].len() {
        most_significants.push(most_significant(&numbers, column));
        least_significants.push(least_significant(&numbers, column));
    }

    to_decimal(&most_significants) * to_decimal(&least_significants)
}

fn part2(numbers: Vec<Vec<u64>>) -> u64 {
    let oxygen_generator_rating = life_support_tracking(numbers.clone(), &most_significant);
    let co2_scrubber_rating = life_support_tracking(numbers.clone(), &least_significant);
    oxygen_generator_rating * co2_scrubber_rating
}

fn life_support_tracking(mut numbers: Vec<Vec<u64>>, f: &dyn Fn(&[Vec<u64>], usize) -> u64) -> u64 {
    let number_len = numbers[0].len();
    let mut column = 0;

    while column < number_len {
        let bit_criteria = f(&numbers, column);

        numbers = numbers
            .into_iter()
            .filter(|v| v[column] == bit_criteria)
            .collect();

        if numbers.len() == 1 {
            return to_decimal(&numbers[0]);
        }

        column += 1;
    }
    unreachable!();
}

fn most_significant(numbers: &[Vec<u64>], position: usize) -> u64 {
    let ones = numbers.into_iter().map(|x| x[position]).sum::<u64>();
    if 2 * ones >= (numbers.len() as u64) {
        1
    } else {
        0
    }
}

fn least_significant(numbers: &[Vec<u64>], position: usize) -> u64 {
    if most_significant(numbers, position) == 1 {
        0
    } else {
        1
    }
}

fn to_decimal(bits: &[u64]) -> u64 {
    bits.into_iter().fold(0, |n, b| n << 1 | b)
}

#[cfg(test)]
mod test {

    use super::{parse, part1, part2};

    #[test]
    fn part1_test() {
        let numbers = parse(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(part1(numbers), 198)
    }

    #[test]
    fn part2_test() {
        let numbers = parse(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(part2(numbers), 230)
    }
}
