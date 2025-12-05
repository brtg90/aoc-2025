use std::cmp::Ordering;

use utils::read_input;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct IdRange {
    start: usize,
    end: usize,
}

impl PartialOrd for IdRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IdRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
            .then_with(|| self.end.cmp(&other.end))
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let (ranges, ids) = parse_ranges_and_ids("inputs/day05pt1.txt");
    let valid_ids = check_valid_ids(&ranges, &ids);
    println!("Part 1: {:?}", valid_ids);
}

fn part_2() {
    let (ranges, _) = parse_ranges_and_ids("inputs/day05pt1.txt");
    let num_valid_ids = count_all_valid_ids(&ranges);
    println!("Part 2: {:?}", num_valid_ids);
}

fn check_valid_ids(ranges: &[IdRange], ids: &[usize]) -> usize {
    // Inclusive range
    ids.iter()
        .filter(|id| {
            ranges.iter().any(|range| id >= &&range.start && id <= &&range.end)
        })
        .count()
}

fn count_all_valid_ids(ranges: &[IdRange]) -> usize {
    let merged = merge_ranges(ranges);
    merged.iter()
        .map(|range| range.end - range.start + 1)
        .sum()
}

fn merge_ranges(ranges: &[IdRange]) -> Vec<IdRange> {
    let mut old: Vec<IdRange> = ranges.to_vec();
    let mut new : Vec<IdRange> = Vec::new();

    old.sort();

    let mut current = old[0];

    for &next in old.iter().skip(1) {
        if current.end >= next.start {
            current.end = current.end.max(next.end);
        }
        else {
            new.push(current);
            current = next;
        }
    }
    new.push(current);

    new
}

fn parse_ranges_and_ids(filename: &str) -> (Vec<IdRange>, Vec<usize>) {
    let input = read_input(filename);

    let mut ranges = vec![];
    let mut ids = vec![];

    let split = input.split("\r\n\r\n").collect::<Vec<&str>>();
    split[0].split("\r\n")
        .for_each(|line| {
            let numbers: Vec<&str> = line.trim().split("-").collect();
            let range = IdRange { start : numbers[0].parse().unwrap(), end: numbers[1].parse().unwrap() };
            ranges.push(range);
        });

    split[1].split("\r\n")
        .for_each(|line| {
            ids.push(line.trim().parse::<usize>().unwrap());
        });

    (ranges, ids)
}