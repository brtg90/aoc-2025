use std::collections::{HashMap, HashSet};
use utils::read_lines;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let boxes = parse("inputs/day08pt2.txt");
    let distances = calculate_distances(&boxes);
    let product = group_boxes(distances, &boxes);

    println!("Part 1: {:?}", product);
}

fn part_2() {
    println!("Part 2: {:?}", 0);
}

fn group_boxes(mut distances: HashMap<usize, HashMap<usize, isize>>, boxes:&[Vec<isize>]) -> usize {
    let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();

    for _ in 0..1000 {
        let (min_idx, min_combination) = find_minimum_distance(&distances);
        println!("Combination found between {:?} and {:?}", boxes[min_combination], boxes[min_idx]);
        let already_containing: Vec<usize> = groups.iter()
            .filter(|(k, v)| v.contains(&min_idx) || v.contains(&min_combination))
            .map(|(k, _)| *k)
            .collect();

        if !already_containing.is_empty() {
            groups.entry(already_containing[0])
                .and_modify(|v| {
                    v.insert(min_idx);
                    v.insert(min_combination);
                });
        }
        groups.entry(min_idx)
            .and_modify(|v| {v.insert(min_combination);})
            .or_insert(HashSet::from([min_combination, min_idx]));

        distances.entry(min_idx)
            .and_modify(|v| {
                v.remove(&min_combination);
            });
    }

    let mut changed = true;
    println!("Groups {:?}", groups);
    while changed {
        changed = false;

        let overlapping = groups.iter()
            .map(|(&k, v)| {
                let overlapping_idx = groups.iter()
                    .filter(|(k1 , v1)| k != **k1 && v.iter().any(|val| v1.contains(val)))
                    .map(|(k1, v1)| {
                        let mut vec = vec![*k1];
                        vec.extend(v1.iter());
                        vec
                    })
                    .collect::<Vec<Vec<usize>>>();
                (k, overlapping_idx)
            })
            .filter(|(_, vec)| !vec.is_empty())
            .collect::<Vec<(usize, Vec<Vec<usize>>)>>();

        if !overlapping.is_empty() {
            changed = true;
            for (idx, v) in overlapping {
                let mut drop: Vec<usize> = vec![];
                for vec in v {
                    groups.entry(idx).and_modify(|value| {
                        let key = vec[0];
                        value.extend(vec);
                        drop.push(key);
                    });
                }
                drop.iter()
                    .for_each(|key| {
                        groups.remove(key);
                    })
            }
        }
    }

    println!("groups: {:?}", groups);

    let mut sizes = groups.values()
        .map(|v| v.len())
        .collect::<Vec<usize>>();

    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

fn find_minimum_distance(distances: &HashMap<usize, HashMap<usize, isize>>) -> (usize, usize) {
    let mut min_distances = distances.iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(&k, v)|
                 {
                     let mut map_pair = v.iter()
                         .map(|(k, v)| (k, v))
                         .collect::<Vec<(&usize, &isize)>>();
                     map_pair.sort_by_key(|(_, v)| *v);
                     (k, map_pair[0])
                 })
        .collect::<Vec<_>>();

    min_distances.sort_by_key(|(_, v)| *v.1);
    (min_distances[0].0, *min_distances[0].1.0)
}

fn calculate_distances(boxes: &[Vec<isize>]) -> HashMap<usize, HashMap<usize, isize>> {
    let mut distances: HashMap<usize, HashMap<usize, isize>> = HashMap::new();
    for (i, box_i) in boxes.iter().enumerate() {
        let mut box_distances: HashMap<usize, isize> = HashMap::new();
        for (j, box_j) in boxes.iter().enumerate().skip(i + 1) {
            box_distances.insert(j, calc_distance_square(box_i, box_j));
        }
        distances.insert(i, box_distances);
    }
    distances
}

fn calc_distance_square(box_1: &[isize], box_2: &[isize]) -> isize {
    (box_2[0] - box_1[0]).pow(2) + (box_2[1] - box_1[1]).pow(2) + (box_2[2] - box_1[2]).pow(2)
}

fn parse(filename: &str) -> Vec<Vec<isize>> {
    read_lines(filename)
        .into_iter()
        .map(|line| {
            line.split(",")
                .map(|str| str.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}