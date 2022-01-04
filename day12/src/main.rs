use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    let caves = parse(&input);

    println!("Part 1: {}", part1(&caves));
    println!("Part 2: {}", part2(&caves));
}

fn part1(caves: &Caves) -> usize {
    count(caves, &mut HashSet::new(), "start", true)
}

fn part2(caves: &Caves) -> usize {
    count(caves, &mut HashSet::new(), "start", false)
}

fn count<'a>(
    caves: &'a Caves,
    visited_small_caves: &mut HashSet<&'a str>,
    position: &'a str,
    has_visited_twice_a_small_cave: bool,
) -> usize {
    caves[position]
        .iter()
        .filter(|&&next_cave| next_cave != "start")
        .map(|next_cave| {
            if *next_cave == "end" {
                return 1;
            }

            if visited_small_caves.contains(next_cave) && has_visited_twice_a_small_cave {
                return 0;
            }

            let will_visited_twice_a_small_cave = has_visited_twice_a_small_cave
                || visited_small_caves.contains(next_cave) && is_small_cave(next_cave);

            let inserted = is_small_cave(next_cave) && visited_small_caves.insert(next_cave);

            let len = count(
                caves,
                visited_small_caves,
                next_cave,
                will_visited_twice_a_small_cave,
            );

            if inserted {
                visited_small_caves.remove(next_cave);
            }

            len
        })
        .sum()
}

type Caves<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> Caves {
    input
        .lines()
        .map(|l| l.split_once('-').expect("Cannot parse cave connexion"))
        .fold(HashMap::new(), |mut acc, (c1, c2)| {
            acc.entry(c1).or_insert_with(Vec::new).push(c2);
            acc.entry(c2).or_insert_with(Vec::new).push(c1);
            acc
        })
}

fn is_small_cave(s: &str) -> bool {
    s.chars()
        .next()
        .expect("Cannot get first letter of a cave")
        .is_lowercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_small_test() {
        let caves = parse(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end
",
        );
        assert_eq!(part1(&caves), 10);
    }

    #[test]
    fn part1_medium_test() {
        let caves = parse(
            "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
",
        );
        assert_eq!(part1(&caves), 19);
    }

    #[test]
    fn part1_large_test() {
        let caves = parse(
            "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
",
        );
        assert_eq!(part1(&caves), 226);
    }

    #[test]
    fn part2_test() {
        let caves = parse(
            "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
",
        );
        assert_eq!(part2(&caves), 3509);
    }
}
