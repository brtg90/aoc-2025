use std::collections::HashMap;

use utils::read_lines;

const NEIGHBOR_STEPS: [(isize, isize);8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];


fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let (papers, (width, height)) = parse_paper_locations("inputs/day04pt1.txt");
    let count: usize = papers.iter()
        .map(|p| get_neighbors(*p, &papers, width, height).len())
        .filter(|n| *n < 4)
        .count();
    println!("Part 1: {:?}", count);
}

fn part_2() {
    let (papers, (width, height)) = parse_paper_locations("inputs/day04pt1.txt");
    let neighbors = get_neighbor_map(&papers, width, height);
    let count = recursive_remove_neighbors(neighbors);
    println!("Part 2: {:?}", count);
}

fn get_neighbor_map(papers: &[(isize, isize)], width: isize, height: isize) -> HashMap<(isize, isize), Vec<(isize, isize)>> {
    let mut map = HashMap::new();
    papers.iter()
        .for_each(|&pos| {
            let neighbors = get_neighbors(pos, papers, width, height);
            map.insert(pos, neighbors);
        });
    map
}

fn recursive_remove_neighbors(mut neighbors: HashMap<(isize, isize), Vec<(isize, isize)>>) -> usize {
    let mut count = 0;

    let remove = neighbors.iter()
        .filter(|(_, neighbors_vec)| neighbors_vec.len() < 4)
        .map(|(&pos, _)| pos)
        .collect::<Vec<_>>();

    if remove.is_empty() {
        return count;
    }

    count += remove.len();

    remove.iter()
        .for_each(|x| {neighbors.remove(x); });

    neighbors.iter_mut()
        .for_each(|(_, neighbor_vec)| {
            neighbor_vec.retain(|x| !remove.contains(x));
        });

    count + recursive_remove_neighbors(neighbors)
}

fn get_neighbors(position: (isize, isize), papers: &[(isize, isize)], width: isize, height: isize) -> Vec<(isize, isize)> {
    let neighbors = get_valid_neighbors(position, width, height);
    neighbors.iter()
        .filter(|neighbor| papers.contains(neighbor))
        .cloned()
        .collect()
}

fn get_valid_neighbors(position: (isize, isize), width: isize, height: isize) -> Vec<(isize, isize)> {
    NEIGHBOR_STEPS.iter()
        .map(|(drow, dcol)| (position.0 + drow, position.1 + dcol))
        .filter(|&(x, y)| x >= 0 && x < height && y >= 0 && y < width)
        .collect()
}

fn parse_paper_locations(filename: &str) -> (Vec<(isize, isize)>, (isize, isize)) {
    let mut paper_locations: Vec<(isize, isize)> = Vec::new();
    let lines = read_lines(filename);
    let height = lines.len();
    let width = lines[0].len();

    lines.into_iter()
        .enumerate()
        .for_each(|(row_no, line)| {
            line.chars()
                .enumerate()
                .for_each(|(col_no, c)| if c == '@' { paper_locations.push((row_no as isize, col_no as isize)) })
        });
    (paper_locations, (width as isize, height as isize))
}