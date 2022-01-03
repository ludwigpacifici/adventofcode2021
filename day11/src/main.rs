use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");
    let grid = parse(&input);
    println!("Part 1: {}", part1(grid.clone()));
    println!("Part 2: {}", part2(grid));
}

#[derive(Clone)]
struct Matrix {
    data: Vec<Vec<u8>>,
    x_len: usize,
    y_len: usize,
}

impl Matrix {
    fn new(data: Vec<Vec<u8>>) -> Matrix {
        let x_len = data[0].len();
        let y_len = data.len();
        Matrix { data, x_len, y_len }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }

    fn is_legal(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || (self.x_len as isize) <= x || (self.y_len as isize) <= y {
            false
        } else {
            true
        }
    }

    fn after_flash(&mut self) -> usize {
        let mut len = 0;
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if 9 < self.data[y][x] {
                    len += 1;
                    self.data[y][x] = 0;
                }
            }
        }
        len
    }

    fn add_all_one(&mut self) {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                self.set(x, y, self.get(x, y) + 1);
            }
        }
    }
}

fn neighbors(x: isize, y: isize) -> [(isize, isize); 8] {
    [
        (x, y - 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
    ]
}

fn tens(g: &Matrix) -> Vec<(usize, usize)> {
    let mut acc = Vec::new();
    for y in 0..g.y_len {
        for x in 0..g.x_len {
            if 9 < g.data[y][x] {
                acc.push((x, y));
            }
        }
    }
    acc
}

fn part1(mut grid: Matrix) -> usize {
    let mut flashes = 0;
    for _ in 0..100 {
        let (g, f) = step(grid);
        flashes += f;
        grid = g;
    }
    flashes
}

fn part2(mut grid: Matrix) -> usize {
    let mut step_number = 1;
    let grid_len = grid.x_len * grid.y_len;
    loop {
        let (g, f) = step(grid);
        if f == grid_len {
            return step_number;
        }
        grid = g;
        step_number += 1;
    }
}

fn step(mut grid: Matrix) -> (Matrix, usize) {
    grid.add_all_one();

    let mut tens = tens(&grid);
    let mut visited = HashSet::new();

    while let Some((x, y)) = tens.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        let neighbors = neighbors(x as isize, y as isize)
            .into_iter()
            .filter(|&(x_n, y_n)| grid.is_legal(x_n, y_n))
            .map(|(x_n, y_n)| (x_n as usize, y_n as usize))
            .collect::<Vec<_>>();

        for (x_n, y_n) in neighbors {
            let value = grid.get(x_n, y_n) + 1;
            grid.set(x_n, y_n, value);
            if 9 < value {
                tens.push((x_n, y_n));
            }
        }

        visited.insert((x, y));
    }

    let flashes = grid.after_flash();
    (grid, flashes)
}

fn parse(input: &str) -> Matrix {
    Matrix::new(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("Cannot convert char to digit") as u8)
                    .collect()
            })
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let grid = parse(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        );

        assert_eq!(part1(grid), 1656);
    }

    #[test]
    fn part2_test() {
        let grid = parse(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        );

        assert_eq!(part2(grid), 195);
    }
}
