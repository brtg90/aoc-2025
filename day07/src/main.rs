use std::collections::{HashMap, HashSet};

use utils::read_lines;

fn main() {
    let (splitters, start, width) = parse("inputs/day07pt1.txt");
    let (visited, endpoints) =  get_visited_splitters_and_endpoints(&splitters, start, width);
    part_1(&visited);
    part_2(visited, endpoints, width);
}

fn part_1(visited: &HashSet<(usize, usize)>) {
    println!("Part 1: {:?}", visited.len());
}

fn part_2(visited: HashSet<(usize, usize)>, endpoints: Vec<(usize, usize)>, width: usize) {
    let mut timeline_map: HashMap<(usize, usize), usize> = HashMap::new();
    let start = visited.iter().min_by_key(|x| x.0).unwrap().to_owned();
    let height = visited.iter().max_by_key(|x| x.0).unwrap().0;

    let timelines = calculate_number_timelines(&visited, &endpoints, start, &mut timeline_map, width, height);
    println!("Part 2: {:?}", timelines);
}

fn calculate_number_timelines(
    visited: &HashSet<(usize, usize)>,
    endpoints: &[(usize, usize)],
    current: (usize, usize),
    timeline_map: &mut HashMap<(usize, usize), usize>,
    width: usize,
    height: usize,
) -> usize {
    let mut timelines = 0;

    if let Some(paths) = timeline_map.get(&current) {
        return *paths;
    }

    let cols = get_valid_neighbors(&current.1, width);

    for col in cols {
        let mut row = current.0;
        let mut count = false;

        while row < height {
            row += 1;

            if visited.contains(&(row, col)) {
                count = true;
                timelines += calculate_number_timelines(visited, endpoints, (row, col), timeline_map, width, height);
                break;
            }

            if endpoints.contains(&(row, col)) {
                timelines += 1;
                count = true;
                break;
            }
        }
        if !count && row >= height {
            timelines += 1;
        }
    }

    timeline_map.insert(current, timelines);

    timelines

}

fn get_visited_splitters_and_endpoints(
    splitters: &[Vec<usize>],
    start: usize, width: usize,
) -> (HashSet<(usize, usize)>, Vec<(usize, usize)>) {
    let mut visited = HashSet::new();

    let mut current = vec![(0, start)];
    for (row_no, row) in splitters.iter().enumerate() {
        for splitter in row {
            let current_cols: Vec<_> = current.iter().map(|(_, col)| *col).collect();
            if current_cols.contains(splitter) {
                get_valid_neighbors(splitter, width)
                    .iter().for_each(|&x| {
                    current.push((row_no, x));
                });
                current.retain(|&x| x.1 != *splitter);
                visited.insert((row_no, *splitter));
            }
        }
    }
    (visited, current)
}

fn get_valid_neighbors(pos: &usize, width: usize) -> Vec<usize> {
    if *pos == 0 {
        return vec![1];
    }
    if *pos == width - 1 {
        return vec![width - 2];
    }
    vec![pos - 1, pos + 1]
}

fn parse(filename: &str) -> (Vec<Vec<usize>>, usize, usize) {
    let mut splitters: Vec<Vec<usize>> = vec![];

    let lines = read_lines(filename);
    let width = lines[0].len();
    let start = lines[0].chars().position(|c| c == 'S').unwrap();
    lines.iter()
        .skip(1)
        .for_each(|line| {
            splitters.push(line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(|(col, _)| col )
                .collect::<Vec<_>>());
        });

    (splitters, start, width)
}