use regex::Regex;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let (numbers, operands) = parse("inputs/day06pt1.txt");
    let sum = get_sum_calculations(&numbers, &operands);
    println!("Part 1: {:?}", sum);
}

fn part_2() {
    let (numbers_str, operands) = parse_numbers_as_string_and_operands("inputs/day06pt1.txt");
    let sum = get_sum_right_to_left_order(&numbers_str, &operands);
    println!("Part 2: {:?}", sum);
}

fn get_sum_right_to_left_order(numbers: &[String], operands: &[u8]) -> usize {
    let mut calculations = vec![];
    let mut operand_idx = 0;

    // Right-to-left order so flip both Vecs
    let operands = operands.iter().rev().collect::<Vec<_>>();
    let numbers = numbers.iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<_>>();

    // Initialize to 1, subtract 1 in case of addition
    let mut value = 1;

    for i in 0..numbers[0].len() {
        let mut value_str = String::new();
        for row in &numbers {
            value_str.push(row.chars().nth(i).unwrap());
        }
        if !value_str.trim().is_empty() {
            let new = value_str.trim().parse::<usize>().unwrap();
            match operands[operand_idx] {
                1 => value *= new,
                _ => value += new,
            }
            if i == numbers[0].len() - 1 {
                if *operands[operand_idx] == 0 {
                    value -= 1;
                }
                calculations.push(value);
            }
        }
        else {
            if *operands[operand_idx] == 0 {
                value -= 1;
            }
            calculations.push(value);
            operand_idx += 1;
            value = 1;
        }

    }
    calculations.iter().sum()
}

fn get_sum_calculations(numbers: &[Vec<usize>], operands: &[u8]) -> usize {
    let mut calculations = numbers[0].to_vec();

    numbers.iter().skip(1)
        .for_each(|vec| {
            vec.iter()
                .enumerate()
                .for_each(|(col, &value)| {
                    if operands[col] == 1 {
                        calculations[col] *= value;
                    }
                    else {
                        calculations[col] += value;
                    }
                });
        });

    calculations.iter().sum()
}

fn parse(filename: &str) -> (Vec<Vec<usize>>, Vec<u8>) {
    let re_nums = Regex::new(r"\d+").unwrap();
    let lines = read_lines(filename);

    let numbers: Vec<Vec<usize>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| re_nums.find_iter(line)
            .map(|m| m.as_str()
                .parse::<usize>()
                .unwrap())
            .collect())
        .collect();

    let operands = parse_operands(&lines[lines.len() - 1]);

    (numbers, operands)
}

fn parse_numbers_as_string_and_operands(filename: &str) -> (Vec<String>, Vec<u8>) {
    let lines = read_lines(filename);
    let operands = parse_operands(&lines[lines.len() - 1]);
    (lines[..lines.len() - 1].to_vec(), operands)
}

fn parse_operands(line: &str) -> Vec<u8> {
    let re_operands = Regex::new(r"[*+]").unwrap();

    re_operands.find_iter(line)
        .map(|m| if m.as_str() == "*" { 1 } else { 0 })
        .collect()
}