use std::collections::HashMap;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let devices = parse("inputs/day11pt1.txt");
    let mut cache: HashMap<String, usize> = HashMap::new();
    let total_paths = count_paths(&devices, String::from("you"), "out", &mut cache);
    println!("Part 1: {:?}", total_paths);
}

fn part_2() {
    let devices = parse("inputs/day11pt1.txt");
    let mut cache: HashMap<String, usize> = HashMap::new();
    // Both orders could be possible, and we don't know which one is correct
    let first = count_paths(&devices, String::from("svr"), "fft", &mut cache);
    cache.clear();
    let second = count_paths(&devices, String::from("fft"), "dac", &mut cache);
    cache.clear();
    let third = count_paths(&devices, String::from("dac"), "out", &mut cache);
    let mut total_paths = first * second * third;

    cache.clear();
    let first = count_paths(&devices, String::from("svr"), "dac", &mut cache);
    cache.clear();
    let second = count_paths(&devices, String::from("dac"), "fft", &mut cache);
    cache.clear();
    let third = count_paths(&devices, String::from("fft"), "out", &mut cache);
    total_paths += first * second * third;
    println!("Part 2: {:?}", total_paths);
}

fn count_paths(devices: &HashMap<String, Vec<String>>, current: String, end: &str, cache: &mut HashMap<String, usize>) -> usize {
    let mut paths = 0;

    if current == end {
        paths += 1;
    }

    if let Some(cached) = cache.get(&current) {
        return *cached;
    }

    if let Some(options) = devices.get(&current) {
        for option in options {
            paths += count_paths(devices, option.to_string(), end, cache);
        }
    }

    cache.insert(current, paths);

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