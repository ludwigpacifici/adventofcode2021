use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    let grid = parse(&input);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

fn parse(input: &str) -> Matrix {
    Matrix::new(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("Cannot convert char to number") as usize)
                    .collect()
            })
            .collect(),
    )
}

fn part1(grid: &Matrix) -> usize {
    lows(grid).into_iter().map(|(x, y)| grid.at(x, y) + 1).sum()
}

fn lows(grid: &Matrix) -> Vec<(usize, usize)> {
    let mut lows = Vec::new();

    for y in 0..grid.y_max {
        for x in 0..grid.x_max {
            let current = grid.at(x, y);
            if grid
                .neighbors(x, y)
                .into_iter()
                .map(|(x_neighbor, y_neighbor)| grid.at(x_neighbor, y_neighbor))
                .all(|neighbors_value| current < neighbors_value)
            {
                lows.push((x, y));
            }
        }
    }

    lows
}

fn part2(grid: &Matrix) -> usize {
    let mut bassins = lows(grid)
        .into_iter()
        .map(|(x, y)| bassin_len(grid, x, y))
        .collect::<Vec<_>>();

    bassins.sort_unstable();

    bassins.into_iter().rev().take(3).product()
}

fn bassin_len(grid: &Matrix, x_low: usize, y_low: usize) -> usize {
    let mut visited = HashSet::new();
    let mut len = 0;
    let mut to_visit = vec![(x_low, y_low)];

    while let Some((x, y)) = to_visit.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        len += 1;

        for (x_n, y_n) in grid.neighbors(x, y) {
            if grid.at(x_n, y_n) != 9 && !visited.contains(&(x_n, y_n)) {
                to_visit.push((x_n, y_n));
            }
        }

        visited.insert((x, y));
    }

    len
}

struct Matrix {
    data: Vec<Vec<usize>>,
    x_max: usize,
    y_max: usize,
}

impl Matrix {
    fn new(data: Vec<Vec<usize>>) -> Matrix {
        let y_max = data.len();
        let x_max = data[0].len();

        Matrix { data, x_max, y_max }
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match (x, y) {
            (0, 0) => vec![(x, y + 1), (x + 1, y)],
            (x, 0) if x == self.x_max - 1 => vec![(x, y + 1), (x - 1, y)],
            (0, y) if y == self.y_max - 1 => vec![(x, y - 1), (x + 1, y)],
            (x, y) if x == self.x_max - 1 && y == self.y_max - 1 => vec![(x, y - 1), (x - 1, y)],
            (x, 0) => vec![(x, y + 1), (x - 1, y), (x + 1, y)],
            (x, y) if y == self.y_max - 1 => vec![(x, y - 1), (x - 1, y), (x + 1, y)],
            (0, y) => vec![(x, y - 1), (x, y + 1), (x + 1, y)],
            (x, y) if x == self.x_max - 1 => vec![(x, y - 1), (x, y + 1), (x - 1, y)],
            (x, y) => vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)],
        }
    }

    fn at(&self, x: usize, y: usize) -> usize {
        self.data[y][x]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let grid = parse(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );

        assert_eq!(part1(&grid), 15);
    }

    #[test]
    fn part2_test() {
        let grid = parse(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );

        assert_eq!(part2(&grid), 1134);
    }
}
