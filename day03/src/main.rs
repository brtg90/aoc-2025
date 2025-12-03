use utils::read_lines;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let banks = parse("inputs/day03pt1.txt");
    let rating = find_banks_joltage(&banks, 2);
    println!("Part 1: {:?}", rating);
}

fn part_2() {
    let banks = parse("inputs/day03pt1.txt");
    let rating = find_banks_joltage(&banks, 12);
    println!("Part 2: {:?}", rating);
}

fn find_banks_joltage(banks: &[Vec<u32>], digits: usize) -> usize {
    banks.iter()
        .map(|bank| {
            let mut start = 0;
            (0..digits).rev()
                .map(|digit| {
                    let result = recursive_maximum_in_slice(&bank[start..], digit);
                    start += result.0 + 1;
                    *result.1 as usize * 10_usize.pow(digit as u32)
                })
                .sum::<usize>()})
            .sum()
}

fn recursive_maximum_in_slice(bank: &[u32], stop: usize) -> (usize, &u32) {
    let end_pos = bank.len() - stop;
    let max = bank[..end_pos].iter().max().unwrap();
    bank[..end_pos].iter().enumerate().find(|(_, v)| v == &max).unwrap()
}

fn parse(filename: &str) -> Vec<Vec<u32>> {
    read_lines(filename)
        .into_iter()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>())
        .collect()
}