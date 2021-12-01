use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    let input = input
        .lines()
        .map(|l| l.parse().expect("Cannot parse integer"))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(s: &[u64]) -> usize {
    count_consecutive_increase(s)
}

fn part2(s: &[u64]) -> usize {
    let x = s.windows(3).map(|w| w.iter().sum()).collect::<Vec<_>>();
    count_consecutive_increase(&x)
}

fn count_consecutive_increase(l: &[u64]) -> usize {
    l.windows(2)
        .filter(|w| {
            let mut it = w.into_iter();
            let first = it.next().expect("Cannot read first element of windows 2");
            let second = it.next().expect("Cannot read second element of windows 2");
            first < second
        })
        .count()
}
