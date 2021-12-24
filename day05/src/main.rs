use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    let coordinates = parse(&input);

    println!("Part 1: {}", part1(&coordinates));
    println!("Part 2: {}", part2(&coordinates));
}

type Point = (isize, isize);

type Line = (Point, Point);

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let coordinates = l
                .split(" -> ")
                .flat_map(|xy| {
                    xy.split(',')
                        .map(|n| n.parse::<isize>().expect("Cannot parse integer"))
                })
                .collect::<Vec<_>>();

            (
                (coordinates[0], coordinates[1]),
                (coordinates[2], coordinates[3]),
            )
        })
        .collect()
}

fn part1(coordinates: &[Line]) -> usize {
    solve(coordinates, &horizontal_or_vertical)
}

fn horizontal_or_vertical(((x0, y0), (x1, y1)): Line) -> bool {
    x0 == x1 || y0 == y1
}

fn part2(coordinates: &[Line]) -> usize {
    solve(coordinates, &always_true)
}

fn always_true(_: Line) -> bool {
    true
}

fn solve(coordinates: &[Line], filter: &dyn Fn(Line) -> bool) -> usize {
    coordinates
        .iter()
        .filter(|&&x| filter(x))
        .fold(HashMap::new(), |mut acc, &coordinates| {
            let ((mut x_bottom, mut y_bottom), (x_top, y_top)) = sort(coordinates);
            let (dx, dy) = increments(x_bottom, y_bottom, x_top, y_top);
            while x_bottom != x_top || y_bottom != y_top {
                *acc.entry((x_bottom, y_bottom)).or_insert(0) += 1;

                x_bottom += dx;
                y_bottom += dy;
            }
            *acc.entry((x_bottom, y_bottom)).or_insert(0) += 1;

            acc
        })
        .values()
        .filter(|&&count| count > 1)
        .count()
}

fn sort(((x0, y0), (x1, y1)): Line) -> Line {
    match y0.cmp(&y1) {
        std::cmp::Ordering::Less => ((x0, y0), (x1, y1)),
        std::cmp::Ordering::Equal => {
            if x0 <= x1 {
                ((x0, y0), (x1, y1))
            } else {
                ((x1, y1), (x0, y0))
            }
        }
        std::cmp::Ordering::Greater => ((x1, y1), (x0, y0)),
    }
}

fn increments(x_bottom: isize, y_bottom: isize, x_top: isize, y_top: isize) -> Point {
    if x_bottom == x_top {
        (0, 1)
    } else if y_bottom == y_top {
        (1, 0)
    } else if x_bottom < x_top {
        (1, 1)
    } else {
        (-1, 1)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let input = parse(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
",
        );

        assert_eq!(part1(&input), 5)
    }

    #[test]
    fn part2_test() {
        let input = parse(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
",
        );

        assert_eq!(part2(&input), 12)
    }
}
