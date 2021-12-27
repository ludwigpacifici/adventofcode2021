use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt").expect("Cannot read file input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split(" | ")
                .nth(1)
                .expect("Cannot read second part of input line")
        })
        .flat_map(|l| l.split_ascii_whitespace())
        .filter(|w| {
            w.len() == 2 // number 1
                || w.len() == 4 // number 4
                || w.len() == 3 // number 7
                || w.len() == 7 // number 8
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(" | ");
            let first_part = it.next().expect("Cannot read first part of input line");
            let second_part = it.next().expect("Cannot read second part of input line");
            (first_part, second_part)
        })
        .map(|(first_part, second_part)| {
            let size_to_segments: HashMap<usize, Vec<&str>> = first_part
                .split_ascii_whitespace()
                .fold(HashMap::new(), |mut acc, w| {
                    acc.entry(w.len()).or_insert_with(Vec::new).push(w);
                    acc
                });

            let mut segments_to_number: HashMap<String, usize> = HashMap::new();

            // (0) uniques by segment count: 1 4 7 8
            let one_as_segment = size_to_segments.get(&2).expect("cannot find number 1")[0];
            segments_to_number.insert(sort(one_as_segment), 1);

            let four_as_segments = size_to_segments.get(&4).expect("cannot find number 4")[0];
            segments_to_number.insert(sort(four_as_segments), 4);

            let seven_as_segments = size_to_segments.get(&3).expect("cannot find number 7")[0];
            segments_to_number.insert(sort(seven_as_segments), 7);

            let eight_as_segments = size_to_segments.get(&7).expect("cannot find number 8")[0];
            segments_to_number.insert(sort(eight_as_segments), 8);

            // (1) find permutation of 0, 6, 9 such that 9 - 7 - 4 = g will
            // give 9 and g because {0, 6} - 7 - 4 = 2 letters.
            let zero_six_nine_as_segments = size_to_segments
                .get(&6) // 0, 6, 9 are size 6
                .expect("Cannot get digits with 6 segments");

            let nine_as_segments = zero_six_nine_as_segments
                .iter()
                .find(|segments| {
                    minus(&minus(segments, seven_as_segments), four_as_segments).len() == 1
                })
                .expect("Cannot deduce number 9");
            segments_to_number.insert(sort(nine_as_segments), 9);

            // (2) 3 - 7 = d + g will give 3 because {2,3} - 7 = 3 letters.
            let two_three_five_as_segments = size_to_segments
                .get(&5) // 2, 3, 5 are size 5
                .expect("Cannot get digits with 5 segments");

            let three_as_segments = two_three_five_as_segments
                .iter()
                .find(|segments| minus(segments, seven_as_segments).len() == 2)
                .expect("Cannot deduce number 3");
            segments_to_number.insert(sort(three_as_segments), 3);

            // (3) 5 - 9 = empty will give 5 and then 2 (last of the same size
            // of 2 and 3).
            let five_as_segments = two_three_five_as_segments
                .iter()
                .filter(|&segments| segments != three_as_segments)
                .find(|segments| minus(segments, nine_as_segments).is_empty())
                .expect("Cannot deduce number 5");
            segments_to_number.insert(sort(five_as_segments), 5);

            let two_as_segments = two_three_five_as_segments
                .iter()
                .find(|&segments| segments != three_as_segments && segments != five_as_segments)
                .expect("Cannot deduce number 2");
            segments_to_number.insert(sort(two_as_segments), 2);

            // (4) 6 - 5 = e will give 6 and 0 (last one).
            let six_as_segments = zero_six_nine_as_segments
                .iter()
                .filter(|&segments| segments != nine_as_segments)
                .find(|segments| minus(segments, five_as_segments).len() == 1)
                .expect("Cannot deduce number 6");
            segments_to_number.insert(sort(six_as_segments), 6);

            let zero_as_segments = zero_six_nine_as_segments
                .iter()
                .find(|&segments| segments != six_as_segments && segments != nine_as_segments)
                .expect("Cannot deduce number 0");
            segments_to_number.insert(sort(zero_as_segments), 0);

            second_part.split_ascii_whitespace().fold(0, |n, segments| {
                let digit = segments_to_number
                    .get(&sort(segments))
                    .expect("Cannot find number for second part");
                10 * n + digit
            })
        })
        .sum()
}

fn minus(on: &str, off: &str) -> String {
    let on = on.chars().collect::<HashSet<_>>();
    let off = off.chars().collect::<HashSet<_>>();
    on.difference(&off).collect()
}

fn sort(s: &str) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort_unstable();
    s.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn part2_test() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(part2(&input), 61229);
    }
}
