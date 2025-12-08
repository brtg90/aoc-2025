use std::collections::{BinaryHeap, HashMap, HashSet};
use utils::read_lines;

type DistanceHeap = BinaryHeap<(isize, usize, usize)>;

fn main() {
    let boxes = parse("inputs/day08pt1.txt");
    let distances = build_distance_heap(&boxes);
    let (groups, distances) = part_1(distances);
    part_2(&boxes, groups, distances);
}

fn part_1(distances: DistanceHeap) -> (HashMap<usize, HashSet<usize>>, DistanceHeap) {
    let (mut groups, distances) = group_boxes_1000_times(distances);
    groups = optimize_groups(groups);
    let product = get_product_largest_groups(&groups);

    println!("Part 1: {:?}", product);

    (groups, distances)
}

fn part_2(boxes: &[Vec<isize>], mut groups: HashMap<usize, HashSet<usize>>, mut distances: DistanceHeap) {
    let mut id_0 = 0;
    let mut id_1 = 0;

    loop {
        if groups.len() == 1 && groups.values().all(|v| v.len() == boxes.len()) {
            break;
        }
        (groups, distances, (id_0, id_1)) = group_boxes(distances, groups);
        groups = optimize_groups(groups);
    }
    let product = boxes[id_0][0] * boxes[id_1][0];
    println!("Part 2: {:?}", product);
}

fn get_product_largest_groups(groups: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut sizes = groups.values()
        .map(|v| v.len())
        .collect::<Vec<usize>>();

    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

fn optimize_groups(mut groups: HashMap<usize, HashSet<usize>>) -> HashMap<usize, HashSet<usize>> {
    let mut changed = true;

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
    groups
}

fn group_boxes(mut distances: DistanceHeap, mut groups: HashMap<usize, HashSet<usize>>) -> (HashMap<usize, HashSet<usize>>, DistanceHeap, (usize, usize)) {
    let (_, min_idx, min_combination) = distances.pop().unwrap();
    let already_containing: Vec<usize> = groups.iter()
        .filter(|(_, v)| v.contains(&min_idx) || v.contains(&min_combination))
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

    (groups, distances, (min_idx, min_combination))
}

fn group_boxes_1000_times(mut distances: DistanceHeap) -> (HashMap<usize, HashSet<usize>>, DistanceHeap) {
    let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();

    for _ in 0..1000 {
        (groups, distances, _) = group_boxes(distances, groups);
    }

    (groups, distances)
}

fn build_distance_heap(boxes: &[Vec<isize>]) -> DistanceHeap {
    let mut heap = BinaryHeap::new();

    for i in 0..boxes.len() {
        for j in i + 1.. boxes.len() {
            let distance = calc_distance_square(&boxes[i], &boxes[j]);
            // The binary heap is a max heap so store negative distance squared
            heap.push((-distance, i, j));
        }
    }
    heap
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