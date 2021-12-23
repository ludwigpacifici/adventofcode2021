use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    let (draws, grids) = parse(&input);

    println!("Part 1: {}", part1(&draws, grids.clone()));
    println!("Part 2: {}", part2(&draws, grids));
}

fn parse(input: &str) -> (Vec<u32>, Vec<Bingo>) {
    let mut it = input.lines();

    let draws: Vec<u32> = it
        .next()
        .expect("Cannot read draw numbers")
        .split(',')
        .map(|n| n.parse().expect("Cannot read a draw number"))
        .collect();

    let grids = it
        .fold(Vec::new(), |mut acc, l| {
            if l.is_empty() {
                acc.push(Vec::new());
            } else {
                acc.last_mut().expect("Cannot get last grid").extend(
                    l.split_whitespace()
                        .map(|n| n.parse::<u32>().expect("Cannot parse grid integer"))
                        .into_iter(),
                );
            }
            acc
        })
        .into_iter()
        .map(Bingo::new)
        .collect();

    (draws, grids)
}

#[derive(Debug, Clone)]
struct Bingo {
    board: u32,
    numbers: HashMap<u32, u32>,
}

impl Bingo {
    fn new(numbers: Vec<u32>) -> Bingo {
        debug_assert_eq!(numbers.len(), 25);

        Bingo {
            board: 0,
            numbers: numbers
                .into_iter()
                .enumerate()
                .map(|(i, n)| (n, 1 << i))
                .collect(),
        }
    }

    fn sum_unmarked_numbers(&self) -> u32 {
        self.numbers
            .iter()
            .filter_map(|(number, &position)| {
                if !self.is_marked(position) {
                    Some(number)
                } else {
                    None
                }
            })
            .sum()
    }

    fn is_winner(&self) -> bool {
        [
            0b00000001111100000000000000000000,
            0b00000000000011111000000000000000,
            0b00000000000000000111110000000000,
            0b00000000000000000000001111100000,
            0b00000000000000000000000000011111,
            0b00000001000010000100001000010000,
            0b00000000100001000010000100001000,
            0b00000000010000100001000010000100,
            0b00000000001000010000100001000010,
            0b00000000000100001000010000100001,
        ]
        .into_iter()
        .any(|position| self.is_marked(position))
    }

    fn is_marked(&self, position: u32) -> bool {
        self.board & position == position
    }

    fn mark(&mut self, number: u32) {
        if let Some(&position) = self.numbers.get(&number) {
            self.board |= position;
        }
    }
}

fn part1(draws: &[u32], mut grids: Vec<Bingo>) -> u64 {
    for d in draws {
        for g in &mut grids {
            g.mark(*d);
            if g.is_winner() {
                return (g.sum_unmarked_numbers() * d).into();
            }
        }
    }
    unreachable!()
}

fn part2(draws: &[u32], mut grids: Vec<Bingo>) -> u64 {
    let mut bingo_players = grids.len();

    for d in draws {
        for g in &mut grids {
            if !g.is_winner() {
                g.mark(*d);
                if g.is_winner() {
                    bingo_players -= 1;
                    if bingo_players == 0 {
                        return (g.sum_unmarked_numbers() * d).into();
                    }
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let (draws, grids) = parse(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
",
        );

        assert_eq!(part1(&draws, grids), 4512)
    }

    #[test]
    fn part2_test() {
        let (draws, grids) = parse(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
",
        );

        assert_eq!(part2(&draws, grids), 1924)
    }
}
