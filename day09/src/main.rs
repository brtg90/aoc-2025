use utils::read_lines;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let grid = parse("inputs/day09pt1.txt");
    let tiles = find_maximum_tiles_between_points(&grid);
    println!("Part 1: {:?}", tiles);
}

fn part_2() {
    println!("Part 2: {:?}", 0);
}

fn find_maximum_tiles_between_points(grid: &[(isize, isize)]) -> isize {
    let mut max = 0;

    for (i, p0) in grid.iter().enumerate() {
        for p1 in grid.iter().skip(i + 1) {
            let tiles = ((p0.0 - p1.0).abs() + 1) * ((p0.1 - p1.1).abs() + 1);
            if tiles > max {
                max = tiles;
            }
        }
    }
    
    max
}

fn parse(filename: &str) -> Vec<(isize, isize)> {
    read_lines(filename)
        .into_iter()
        .map(|line| {
            let mut iter = line.split(",")
                .map(|str| str.parse::<isize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
    .collect()
}