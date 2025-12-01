use utils::read_lines;


fn main() {
    part_1();
    part_2();
}

#[derive(Debug)]
enum Instruction {
    Left(isize),
    Right(isize),
}

fn part_1() {
    let input = parse("inputs/day01pt1.txt");
    let end_zeros = count_zero_endpoints_in_cycle(&input);
    println!("Part 1: {:?}", end_zeros);
}

fn part_2() {
    let input = parse("inputs/day01pt1.txt");
    let answer = solve_part_two(&input);
    println!("Part 2: {:?}", answer);
}

fn count_zero_endpoints_in_cycle(instructions: &[Instruction]) -> usize {
    let mut current = 50;
    let mut end_counts = 0;

    instructions.iter()
        .for_each(|instruction| {
            let old = current;
            match instruction {
                Instruction::Left(n) => {
                    current -= n;
                },
                Instruction::Right(n) => {
                    current += n;
                },
            }
            current = current.rem_euclid(100);
            if current == 0 {
                end_counts += 1;
            }
        });

    end_counts
}

fn solve_part_two(instructions: &[Instruction]) -> isize {
    let mut hits = 0;
    let mut current = 50;

    instructions.iter()
        .for_each(|instruction| {
            let dist : isize;

            match instruction {
                Instruction::Left(n) => {
                    // Starting from 0 the distance is different since it will not cross 0 initially
                    if current == 0 {
                        dist = 100;
                    }
                    else {
                        dist = current;
                    }
                    if *n >= dist {
                        hits += 1 + (n - dist)/ 100;
                    }
                    current -= n;
                },
                Instruction::Right(n) => {
                    dist = 100 - current;
                    if *n >= dist {
                        hits += 1 + (n - dist) / 100;
                    }
                    current += n;
                }
            }
            current = current.rem_euclid(100);

        });
    hits
}

fn parse(filename: &str) -> Vec<Instruction> {
    read_lines(filename)
        .into_iter()
        .map(|line| {
            let num = line[1..].parse::<isize>().unwrap();
            if line.chars().next().unwrap() == 'L' {
                Instruction::Left(num)
            }
            else {
                Instruction::Right(num)
            }
        })
        .collect()
}