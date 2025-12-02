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
    println!("Part 2: {:?}", 0);
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
