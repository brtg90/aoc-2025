use std::collections::HashMap;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let devices = parse("inputs/day11pt1.txt");
    let total_paths = count_paths(&devices, String::from("you"));
    println!("Part 1: {:?}", total_paths);
}

fn part_2() {
    println!("Part 2: {:?}", 0);
}

fn count_paths(devices: &HashMap<String, Vec<String>>, current: String) -> usize {
    let mut paths = 0;

    if current == String::from("out") {
        paths += 1;
    }

    if let Some(options) = devices.get(&current) {
        for option in options {
            paths += count_paths(devices, option.to_string());
        }
    }
    paths
}

fn parse(filename: &str) -> HashMap<String, Vec<String>> {
    let mut devices: HashMap<String, Vec<String>> = HashMap::new();

    read_lines(filename)
        .into_iter()
        .for_each(|line| {
            let split : Vec<&str> = line.split(": ").collect();
            let key = split[0].to_string();
            let value = split[1].split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            devices.insert(key, value);
        });
    devices
}