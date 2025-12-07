use utils::read_lines;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let (splitters, start, width) = parse("inputs/day07pt1.txt");
    let beams = count_splitted_beams(&splitters, start, width);
    println!("Part 1: {:?}", beams);
}

fn part_2() {
    println!("Part 2: {:?}", 0);
}

fn count_splitted_beams(splitters: &[Vec<usize>], start: usize, width: usize) -> usize {
    let mut beams = 0;

    let mut current = vec![start];
    for row in splitters {
        for splitter in row {
            if current.contains(splitter) {
                beams += 1;
                get_valid_neighbors(splitter, width)
                    .iter().for_each(|&x| current.push(x));
                current.retain(|&x| x != *splitter);
            }
        }
    }
    beams
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