use std::env::var;
use num_rational::Rational64;
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

    fn solve_joltage_presses(&self) -> usize {
        let rref = self.get_rref();
        let pivots = Self::get_pivots(&rref);
        println!("pivots: {:?}", pivots);
        let constraints = Self::get_constraints(&rref, &pivots);
        0
    }

    fn get_rref(&self) -> Vec<Vec<Rational64>> {
        let num_lights = self.get_num_lights();
        let mut rref: Vec<Vec<Rational64>> = vec![vec![Rational64::from_integer(0); self.buttons.len() + 1]; num_lights + 1];

        // Initialize values
        for (col, button) in self.buttons.iter().enumerate() {
            button.iter()
                .for_each(|&n| {
                rref[n][col] = Rational64::from_integer(1);
            });
        }

        self.joltage.iter()
            .enumerate()
            .for_each(|(i, &l)| rref[i][self.buttons.len()] = Rational64::from_integer(l as i64));
        // println!("Original: {:?}", Self::convert_rref_to_int(&rref));

        rref = Self::calculate_reduced_row_echelon_form(rref);
        // println!("{:?}", rref);
        // println!("Pivots {:?}", Self::get_pivots(&rref));
        rref
    }

    fn calculate_reduced_row_echelon_form(mut rref: Vec<Vec<Rational64>>) -> Vec<Vec<Rational64>> {
        let mut pivot = 0;
        let rows = rref.len();
        let cols = rref[0].len();

        'outer: for row in 0..rows {
            if pivot >= cols {
                break;
            }
            let mut row_comp = row;
            while rref[row_comp][pivot].numer() == &0 {
                row_comp += 1;
                if row_comp == rows {
                    row_comp = row;
                    pivot += 1;
                    if pivot == cols {
                        break 'outer;
                    }
                }
            }
            rref.swap(row, row_comp);
            let value = rref[row][pivot];
            if value.numer() != &0 {
                for col in 0..cols {
                    rref[row][col] /= value;
                }
            }

            for row_i in 0..rows {
                if row_i == row {
                    continue;
                }
                let value = rref[row_i][pivot];
                for col in 0..cols {
                    rref[row_i][col] = rref[row_i][col] - value * rref[row][col];
                }
            }
            pivot += 1;
        }

        rref
    }

    fn get_constraints(rref: &[Vec<Rational64>], pivots: &[usize]) -> Vec<usize> {
        let num_free = rref[0].len() - pivots.len();
        let mut constraints: Vec<usize> = vec![usize::MAX; num_free];
        let mut pivot_idx = 0;

        rref.iter()
            .for_each(|row| {
                let pivot = pivots[pivot_idx];
                if row[pivot].numer() != &0 {
                    let variables = row.iter()
                        .enumerate()
                        .filter(|(col, x)| x.numer() != &0 && *col != pivot)
                        .map(|(_, x)| *x)
                        .collect::<Vec<Rational64>>();
                    println!("variables: {:?}", variables);
                    // let constraint = variables[variables.len() - 1] - ;
                    pivot_idx += 1;
                }
            });

        constraints
    }

    fn get_pivots(rref: &[Vec<Rational64>]) -> Vec<usize> {
        let mut pivots: Vec<usize> = vec![];
        let rows = rref.len();
        for col in 0..rref[0].len() {
            let mut non_zero = 0;
            for row in 0..rows {
                if rref[row][col].numer() != &0 {
                    non_zero += 1;
                }
            }
            if non_zero == 1 {
                pivots.push(col);
            }
        }
        pivots
    }

    fn convert_rref_to_int(rref: &[Vec<Rational64>]) -> Vec<Vec<isize>> {
        rref.iter()
            .map(|row| {
                row.iter()
                    .map(|x| if x.is_integer() { x.to_integer() as isize } else {
                        println!("x: {:?}", x);
                        panic!("invalid value found!"); })
                    .collect()
            })
        .collect()

    }


    // fn print_rref(rref: &Vec<Vec<Rational64>>) {
    //     for row in rref {
    //         for col in row {
    //             print!("{}\t", col.to_integer());
    //         }
    //         println!();
    //     }
    // }
}

fn main() {
    let machines = parse_machines("inputs/day10pt2.txt");
    part_1(&machines);
    part_2(&machines);
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

fn part_2(machines: &[Machine]) {
    for machine in machines {
        machine.solve_joltage_presses();
    }
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
