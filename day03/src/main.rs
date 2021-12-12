use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");
    let (len, numbers) = parse(&input);

    println!("Part 1: {}", part1(len, &numbers));
    println!("Part 2: {}", part2(len, numbers));
}

fn parse(input: &str) -> (usize, Vec<u32>) {
    let len = input
        .lines()
        .next()
        .expect("Cannot read the first input  line")
        .len();
    let numbers = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).expect("Cannot convert string binary to integer"))
        .collect();
    (len, numbers)
}

fn part1(number_len: usize, numbers: &[u32]) -> u32 {
    let most_common_bits = (0..number_len)
        .rev()
        .map(|position| most_significant(numbers, position))
        .fold(0, |acc, most_significant_bit| {
            acc << 1 | most_significant_bit
        });

    let least_significant_bits = !(u32::MAX << number_len) ^ most_common_bits;

    least_significant_bits * most_common_bits
}

fn part2(number_len: usize, numbers: Vec<u32>) -> u32 {
    let oxygen_generator_rating =
        life_support_tracking(number_len, numbers.clone(), &most_significant);
    let co2_scrubber_rating = life_support_tracking(number_len, numbers, &least_significant);
    oxygen_generator_rating * co2_scrubber_rating
}

fn life_support_tracking(
    number_len: usize,
    mut numbers: Vec<u32>,
    f: &dyn Fn(&[u32], usize) -> u32,
) -> u32 {
    let mut column = number_len - 1;

    loop {
        let bit_criteria = f(&numbers, column);

        numbers = numbers
            .into_iter()
            .filter(|v| ((v >> column) & 1) == bit_criteria)
            .collect();

        if numbers.len() == 1 {
            return numbers[0];
        }

        column -= 1;
    }
}

fn most_significant(numbers: &[u32], position: usize) -> u32 {
    let column_mask = 1 << position;
    let one_count = numbers.iter().filter(|&n| column_mask & n != 0).count();

    if 2 * one_count >= numbers.len() {
        1
    } else {
        0
    }
}

fn least_significant(numbers: &[u32], position: usize) -> u32 {
    if most_significant(numbers, position) == 1 {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let (len, numbers) = parse(
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
        assert_eq!(part1(len, &numbers), 198)
    }

    #[test]
    fn part2_test() {
        let (len, numbers) = parse(
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
        assert_eq!(part2(len, numbers), 230)
    }
}
