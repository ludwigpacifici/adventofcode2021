use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(status)
        .filter_map(|s| match s {
            Status::Illegal(c) => Some(points1(c)),
            Status::Incomplete(_) => None,
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut points = input
        .lines()
        .map(status)
        .filter_map(|s| match s {
            None => None,
            Some(Status::Illegal(_)) => None,
            Some(Status::Incomplete(leftovers)) => Some(
                leftovers
                    .into_iter()
                    .rev()
                    .fold(0, |acc, c| acc * 5 + points2(c)),
            ),
        })
        .collect::<Vec<_>>();

    points.sort_unstable();

    points[points.len() / 2]
}

enum Status {
    Illegal(char),
    Incomplete(Vec<char>),
}

fn status(l: &str) -> Option<Status> {
    let mut stack = Vec::new();

    for c in l.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(open) = stack.pop() {
                    if !is_match_delimiter(open, c) {
                        return Some(Status::Illegal(c));
                    }
                }
            }
            c => panic!("Unknown delimiter {}", c),
        }
    }

    if stack.is_empty() {
        None
    } else {
        Some(Status::Incomplete(stack))
    }
}

fn is_match_delimiter(open: char, close: char) -> bool {
    matches!(
        (open, close),
        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>')
    )
}

fn points1(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        c => panic!("Unknown points1 for {}", c),
    }
}

fn points2(c: char) -> usize {
    match c {
        '(' | ')' => 1,
        '[' | ']' => 2,
        '{' | '}' => 3,
        '<' | '>' => 4,
        c => panic!("Unknown points2 for {}", c),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn part2_test() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(part2(&input), 288957);
    }
}
