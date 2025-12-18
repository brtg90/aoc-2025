use std::collections::HashMap;

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
        // println!("pivots: {:?}", pivots);
        let mut detailed = false;
        if self.joltage == vec![97, 74, 93, 57, 65, 79, 46, 51, 84, 79] ||
            self.joltage == vec![63, 64, 76, 48, 59, 65, 47, 43, 55, 38] {
            detailed = true;
        }
        let constraints = self.get_constraints(&rref, &pivots);
        if detailed {
            println!("constraints: {:?}", constraints);
        }

        let mut free_vars: Vec<usize> = constraints.keys().copied().collect();
        free_vars.sort();

        let mut row_to_pivot = HashMap::new();
        for (r_idx, row) in rref.iter().enumerate() {
            if let Some(c_idx) = row.iter().position(|val| val.numer() != &0) {
                // This excludes rows with all zeros and rows with all zeros but the target, i.e.,
                // invalid rows
                if pivots.contains(&c_idx) {
                    row_to_pivot.insert(r_idx, c_idx);
                }
            }
        }

        let mut min_total_presses = usize::MAX;
        let mut current_assignments = HashMap::new();

        self.recursive_solve(
            0,
            &free_vars,
            &mut current_assignments,
            &constraints,
            &rref,
            &row_to_pivot,
            &mut min_total_presses,
        );
        if min_total_presses == usize::MAX {
            println!("Machine: {:?}", self.joltage);
            println!("rref : {:?}", Self::print_rref(&rref));
        }
        // println!("current assignments: {:?}", current_assignments);
        // let truth = self.verify_solution(&current_assignments);
        // println!("Good solution?: {}", truth);

        println!("Presses needed: {min_total_presses:#?}");
        min_total_presses

    }

    fn recursive_solve(
        &self,
        free_idx: usize,
        free_vars: &[usize],
        assignments: &mut HashMap<usize, usize>,
        constraints: &HashMap<usize, usize>,
        rref: &[Vec<Rational64>],
        row_to_pivot: &HashMap<usize, usize>,
        min_total: &mut usize,
    ) {
        // Base Case: All free variables have been assigned a value
        if free_idx == free_vars.len() {
            let mut current_sum = assignments.values().sum::<usize>();

            // Pruning: if current sum already exceeds min, stop
            if current_sum >= *min_total { return; }

            // Calculate the values for all PIVOT variables based on these free variables
            for (&row_idx, &pivot_col) in row_to_pivot {
                let row = &rref[row_idx];
                let target = row[row.len() - 1];

                // Equation: Pivot = Target - Sum(coeff * FreeVariable)
                let mut pivot_val = target;
                for &f_col in free_vars {
                    let coeff = row[f_col];
                    let f_val = Rational64::from_integer(assignments[&f_col] as i64);
                    pivot_val -= coeff * f_val;
                }

                // Constraint Check: Pivot must be a non-negative integer
                if pivot_val.numer() < &0 || !pivot_val.is_integer() {
                    return; // Invalid combination
                }

                current_sum += pivot_val.to_integer() as usize;
            }

            // If we made it here, it's a valid solution!
            if current_sum < *min_total {
                *min_total = current_sum;
            }
            return;
        }

        // Recursive Step: Try all values for the current free variable
        let f_col = free_vars[free_idx];
        let max_val = constraints[&f_col];

        for val in 0..=max_val {
            assignments.insert(f_col, val);
            self.recursive_solve(
                free_idx + 1,
                free_vars,
                assignments,
                constraints,
                rref,
                row_to_pivot,
                min_total,
            );

            // Optimization: if even the smallest sum of free variables
            // exceeds min_total, you could break early here.
        }
    }

    fn verify_solution(&self, solution: &HashMap<usize, usize>) -> bool {
        let mut actual_joltage = vec![0; self.joltage.len()];

        for (&btn_idx, &presses) in solution {
            let buttons_lights = &self.buttons[btn_idx];
            for &light_idx in buttons_lights {
                if light_idx < actual_joltage.len() {
                    actual_joltage[light_idx] += presses;
                }
            }
        }
        println!("{:?}", actual_joltage);
        println!("{:?}", self.joltage);

        actual_joltage == self.joltage
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

    // fn get_constraints(&self, pivots: &[usize]) -> HashMap<usize, (usize, usize)> {
    //     let mut constraints = HashMap::new();
    //     let num_buttons = self.buttons.len();
    //
    //     // Most puzzles of this type don't require pressing a single button
    //     // more than 100 times. If you still get MAX, try 200.
    //     let search_limit = 100;
    //
    //     for col_idx in 0..num_buttons {
    //         if !pivots.contains(&col_idx) {
    //             // Simply allow the search to explore 0..search_limit
    //             constraints.insert(col_idx, (0, search_limit));
    //         }
    //     }
    //     constraints
    // }

    fn get_constraints(&self, rref: &[Vec<Rational64>], pivots: &[usize]) -> HashMap<usize, usize> {
        let mut constraints: HashMap<usize, usize> = HashMap::new();
        let num_buttons = self.buttons.len();

        // Initialize all free variables with a sensible global limit.
        // A button can never be pressed more times than the max joltage requirement.
        let global_max = self.joltage.iter().max().cloned().unwrap_or(0);

        for col_idx in 0..num_buttons {
            if !pivots.contains(&col_idx) {
                constraints.insert(col_idx, global_max);
            }
        }

        for row in rref {
            let target = row[num_buttons];
            // If the target is negative, this specific RREF state might be tricky,
            // but for these puzzles, targets are usually non-negative.
            if target.numer() < &0 {
                continue;
            }
            for col_idx in 0..num_buttons {
                if !pivots.contains(&col_idx) {
                    let coeff = row[col_idx];
                    // Only a positive coefficient limits how LARGE a free variable can be.
                    if coeff.numer() > &0 {
                        let limit = (target / coeff).to_integer() as usize;
                        constraints.entry(col_idx).and_modify(|e| *e = (*e).min(limit));
                    }
                }
            }
        }
        constraints
    }

    // fn get_constraints(&self, rref: &[Vec<Rational64>], pivots: &[usize]) -> HashMap<usize, usize> {
    //     let mut constraints: HashMap<usize, usize> = HashMap::new();
    //     (0..self.buttons.len())
    //         .filter(|x| !pivots.contains(x))
    //         .for_each(|x| {
    //             constraints.insert(x, usize::MAX);
    //         });
    //
    //     let mut order = vec![];
    //     for (i, row) in rref.iter().enumerate() {
    //         let counts = row.iter()
    //             .enumerate()
    //             .filter(|(j, x)| !pivots.contains(j) && x.numer() != &0)
    //             .count();
    //         if counts != 0 {
    //             order.push((i, counts));
    //         }
    //     }
    //     order.sort_by_key(|x| x.1);
    //
    //     for i in 0..order.len() {
    //         let row_idx = order[i].0;
    //         let row = &rref[row_idx];
    //         let target = row[row.len() - 1];
    //         let components = row[..row.len() -1].iter()
    //             .enumerate()
    //             .filter(|(col, x)| x.numer() != &0 && !pivots.contains(col))
    //             .collect::<Vec<_>>();
    //         for (i, component) in &components {
    //             let new_target = target - components.iter()
    //                 .filter(|(j, c)| c!= component)
    //                 .map(|(j, c)| Rational64::from_integer(constraints[j] as i64) * row[*j])
    //                 .sum::<Rational64>();
    //             let mut max: Rational64 = new_target / *component;
    //             if !max.is_integer() {
    //                 let denom = max.denom();
    //                 max = max * denom;
    //                 assert_eq!(max.is_integer(), true);
    //             }
    //             let mut max = max.to_integer() as usize;
    //             constraints.entry(*i)
    //                 .and_modify(|e| *e = *e.min(&mut max));
    //         }
    //     }
    //     // println!("constraints: {:?}", constraints);
    //     assert!(!constraints.values().any(|v| *v == usize::MAX));
    //
    //     constraints
    // }

    fn get_pivots(rref: &[Vec<Rational64>]) -> Vec<usize> {
        let mut pivots = Vec::new();
        for row in rref {
            if let Some(col_idx) = row.iter().take(row.len() - 1).position(|x| x.numer() != &0) {
                pivots.push(col_idx);
            }
        }
        pivots
    }

    // fn get_pivots(rref: &[Vec<Rational64>]) -> Vec<usize> {
    //     let mut pivots: Vec<usize> = vec![];
    //     let rows = rref.len();
    //     for col in 0..rref[0].len() {
    //         let mut non_zero = 0;
    //         for row in 0..rows {
    //             if rref[row][col].numer() != &0 {
    //                 non_zero += 1;
    //             }
    //         }
    //         if non_zero == 1 {
    //             pivots.push(col);
    //         }
    //     }
    //     pivots
    // }

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

    fn print_rref(rref: &[Vec<Rational64>]) {
        for row in rref {
            for col in row {
                print!("{}\t", col.to_string());
            }
            println!();
        }
    }
}

fn main() {
    let machines = parse_machines("inputs/day10pt3.txt");
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
