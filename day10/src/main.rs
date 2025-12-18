use std::collections::HashMap;

use num_rational::Rational64;
use regex::Regex;

use utils::read_lines;

struct RowConstraint {
    target: Rational64,
    free_coeffs: Vec<(usize, Rational64)>,
}

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
        let num_buttons = self.buttons.len();

        let constraints_map = self.get_constraints(&pivots);
        let mut free_vars = Vec::new();
        let mut max_presses = vec![0; num_buttons];
        for (&col, &max) in &constraints_map {
            free_vars.push(col);
            max_presses[col] = max;
        }
        free_vars.sort();

        let mut row_constraints = Vec::new();
        for row in rref.iter() {
            if let Some(p_col) = row.iter().take(num_buttons).position(|v| v.numer() != &0)
                && pivots.contains(&p_col) {
                    let mut free_coeffs = Vec::new();
                    for &f_col in &free_vars {
                        if row[f_col].numer() != &0 {
                            free_coeffs.push((f_col, row[f_col]));
                        }
                    }
                    row_constraints.push(RowConstraint {
                        target: row[num_buttons],
                        free_coeffs,
                    });
                }
        }
        let mut min_total_presses = usize::MAX;
        let mut assignments = vec![0; num_buttons];

        Self::recursive_solve(
            0,
            &free_vars,
            &mut assignments,
            &max_presses,
            &row_constraints,
            0,
            &mut min_total_presses,
        );

        min_total_presses
    }

    fn recursive_solve(
        free_idx: usize,
        free_vars: &[usize],
        assignments: &mut [usize],
        max_presses: &[usize],
        row_constraints: &[RowConstraint],
        current_sum: usize,
        min_total: &mut usize,
    ) {
        if current_sum >= *min_total {
            return;
        }

        if free_idx == free_vars.len() {
            let mut total_with_pivots = current_sum;

            for rc in row_constraints {
                let mut pivot_val = rc.target;
                for &(f_col, coeff) in &rc.free_coeffs {
                    pivot_val -= coeff * Rational64::from_integer(assignments[f_col] as i64);
                }

                if pivot_val.numer() < &0 || !pivot_val.is_integer() {
                    return;
                }

                total_with_pivots += pivot_val.to_integer() as usize;

                if total_with_pivots >= *min_total {
                    return;
                }
            }

            if total_with_pivots < *min_total {
                *min_total = total_with_pivots;
            }
            return;
        }

        let f_col = free_vars[free_idx];
        let max_val = max_presses[f_col];

        for val in 0..=max_val {
            assignments[f_col] = val;
            Self::recursive_solve(
                free_idx + 1,
                free_vars,
                assignments,
                max_presses,
                row_constraints,
                current_sum + val,
                min_total,
            );
        }
    }


    fn get_rref(&self) -> Vec<Vec<Rational64>> {
        let num_lights = self.get_num_lights();
        let mut rref: Vec<Vec<Rational64>> = vec![vec![Rational64::from_integer(0); self.buttons.len() + 1]; num_lights + 1];

        for (col, button) in self.buttons.iter().enumerate() {
            button.iter()
                .for_each(|&n| {
                rref[n][col] = Rational64::from_integer(1);
            });
        }

        self.joltage.iter()
            .enumerate()
            .for_each(|(i, &l)| rref[i][self.buttons.len()] = Rational64::from_integer(l as i64));

        rref = Self::calculate_reduced_row_echelon_form(rref);
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

    fn get_constraints(&self, pivots: &[usize]) -> HashMap<usize, usize> {
        let mut constraints: HashMap<usize, usize> = HashMap::new();
        let num_buttons = self.buttons.len();

        // A button can never be pressed more times than the max joltage requirement.
        let global_max = self.joltage.iter().max().cloned().unwrap_or(0);

        for col_idx in 0..num_buttons {
            if !pivots.contains(&col_idx) {
                constraints.insert(col_idx, global_max);
            }
        }

        constraints
    }

    fn get_pivots(rref: &[Vec<Rational64>]) -> Vec<usize> {
        let mut pivots = Vec::new();
        for row in rref {
            if let Some(col_idx) = row.iter().take(row.len() - 1).position(|x| x.numer() != &0) {
                pivots.push(col_idx);
            }
        }
        pivots
    }
}

fn main() {
    let machines = parse_machines("inputs/day10pt1.txt");
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
    let presses = machines.iter()
        .map(|machine| machine.solve_joltage_presses())
        .sum::<usize>();
    println!("Part 2: {:?}", presses);
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
