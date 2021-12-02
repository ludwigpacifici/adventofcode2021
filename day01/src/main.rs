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
    solve(s, 2)
}

fn part2(s: &[u64]) -> usize {
    // a + b + c < b + c + d <=> a < d
    solve(s, 4)
}

fn solve(s: &[u64], n: usize) -> usize {
    s.windows(n).filter(|w| w[0] < w[n - 1]).count()
}
