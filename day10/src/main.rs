use regex::Regex;

use utils::read_lines;

#[derive(Debug)]
struct Machine {
    lights: usize,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_string(line: &str) -> Machine {
        let re_lights = Regex::new(r"\[[.#]+]").unwrap();
        let re_buttons = Regex::new(r"\([\d,]+\)").unwrap();
        let re_joltage = Regex::new(r"\{[\d,]+}").unwrap();

        let light_caps = re_lights.captures(line).unwrap();
        let button_caps = re_buttons.captures_iter(line);
        let joltage_caps = re_joltage.captures(line).unwrap();

        let lights = light_caps.get(0)
            .unwrap()
            .as_str()
            .chars()
            .filter(|c| *c != '[' && *c != ']')
            .rev()
            .enumerate()
            .map(|(i, c)| if c == '#' { 1 << i } else { 0 })
            .sum();

        let buttons = button_caps
            .map(|cap| {
                let mut str = cap.get(0).unwrap().as_str();
                str = &str[1..str.len() - 1];
                str.split(",").collect::<Vec<&str>>()
                    .iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();

        let joltage_str = joltage_caps.get(0)
            .unwrap()
            .as_str();
        let joltage = joltage_str[1..joltage_str.len() - 1]
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Machine { lights, buttons, joltage }
    }

    fn get_num_lights(&self) -> usize {
        *self.buttons.iter().flatten().max().unwrap()
    }
}

fn main() {
    let machines = parse_machines("inputs/day10pt1.txt");
    part_1(&machines);
    part_2();
}

fn parse_machines(filename: &str) -> Vec<Machine> {
    read_lines(filename)
        .into_iter()
        .map(|line| Machine::from_string(&line))
        .collect::<Vec<Machine>>()
}

fn part_1(machines: &[Machine]) {
    let sum: usize = machines.iter()
        .map(get_minimum_number_presses)
        .sum();
    println!("Part 1: {:?}", sum);
}

fn part_2() {
    println!("Part 2: {:?}", 0);
}

fn get_minimum_number_presses(machine: &Machine) -> usize {
    let num_buttons = machine.buttons.len();
    let max_combinations: usize = 1 << num_buttons;
    let num_lights = machine.get_num_lights();

    let mut min_presses = usize::MAX;

    let button_masks: Vec<usize> = machine.buttons.iter()
        .map(|indices| {
            indices.iter()
                .fold(0, |acc, &b| acc | (1 << (num_lights - b)))
        })
        .collect();

    for i in 0..max_combinations {
        let mut current = 0;
        let mut pressed = vec![];
        for j in 0..num_buttons {
            if (i & (1 << j)) != 0 {
                pressed.push(&machine.buttons[j]);
                current ^= button_masks[j];
            }
        }
        if current == machine.lights {
            let presses = i.count_ones() as usize;
            min_presses = min_presses.min(presses);
        }
    }
    min_presses
}
