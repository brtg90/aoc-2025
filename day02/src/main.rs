use std::collections::HashMap;

use utils::read_input;

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let ranges = parse("inputs/day02pt1.txt");
    let sum: usize = ranges.iter()
        .map(|range| get_invalid_double_ids_from_range(range).iter().sum::<usize>())
        .sum();
    println!("Part 1: {:?}", sum);
}

fn part_2() {
    let ranges = parse("inputs/day02pt1.txt");
    let sum: usize = ranges.iter()
        .map(|range| get_all_invalid_ids_from_range(range).iter().sum::<usize>())
        .sum();
    println!("Part 2: {:?}", sum);
}

fn get_invalid_double_ids_from_range(range: &Range) -> Vec<usize> {
    (range.start..=range.end)
        .map(|i| i.to_string())
        .filter(|s| {
            let len = s.len();
            len % 2 == 0 && s[0..len / 2] == s[len / 2..]
        })
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_all_invalid_ids_from_range(range: &Range) -> Vec<usize> {
    (range.start..=range.end)
        .map(|i| i.to_string())
        .filter(|s| {
            let mut unique_chars: HashMap<char, usize> = HashMap::new();
            s.chars().
                for_each(|c| {
                    unique_chars.entry(c)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                });
            let min_repeats = unique_chars.values().min().unwrap();

            if *min_repeats == 1 {
                return false
            }

            // The code below finds anything based on chunks consisting of length s.len() / min_repeats
            // This fails for edge cases where the min_repeat number is included twice into the
            // duplicated part, e.g. 12211221. To include these, simply look at the old method
            if s[0..s.len() / 2] == s[s.len() / 2..] {
                return true;
            }

            let chars = s.chars().collect::<Vec<char>>();
            let chunks = chars.chunks(s.len() / min_repeats).collect::<Vec<&[char]>>();
            chunks.iter()
                .all(|ch| ch == &chunks[0])
        })
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse(filename: &str) -> Vec<Range> {
    read_input(filename)
        .split(",")
        .map(|s| {
            let mut split = s.split("-");
            let start = split.next().unwrap().parse::<usize>().unwrap();
            let end = split.next().unwrap().parse::<usize>().unwrap();
            Range { start, end }
        })
        .collect()
}
