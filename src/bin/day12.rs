use std::{collections::HashSet, fmt::Display};

type Coord = i32;
type Loc = [Coord; 2];
type Brick = HashSet<Loc>;

struct Region {
    size: Loc,
    requirements: Vec<usize>,
}

struct Input {
    bricks: Vec<Brick>,
    regions: Vec<Region>,
}

fn parse_input(input: &str) -> Input {
    let sections: Vec<_> = input.split("\n\n").collect();

    let bricks = sections[..sections.len() - 1]
        .iter()
        .map(|section| {
            section
                .lines()
                .skip(1)
                .enumerate()
                .flat_map(|(i, row)| {
                    row.chars().enumerate().filter_map(move |(j, c)| {
                        if c == '#' {
                            Some([i as Coord, j as Coord])
                        } else {
                            None
                        }
                    })
                })
                .collect()
        })
        .collect();

    let trees = sections[sections.len() - 1]
        .lines()
        .map(|line| {
            let (size, requirements) = line.split_once(':').unwrap();

            let (x, y) = size.split_once('x').unwrap();
            let size = [x.parse().unwrap(), y.parse().unwrap()];

            let requirements = requirements
                .trim()
                .split_whitespace()
                .map(|token| token.parse().unwrap())
                .collect();

            Region { size, requirements }
        })
        .collect();

    Input {
        bricks,
        regions: trees,
    }
}

fn solve_part1(input: &Input) -> impl Display {
    let counts: Vec<_> = input.bricks.iter().map(|b| b.len()).collect();
    input
        .regions
        .iter()
        .filter(|&region| {
            let volume_min = region
                .requirements
                .iter()
                .zip(counts.iter())
                .map(|(r, c)| r * c)
                .sum::<usize>();
            volume_min <= (region.size[0] as usize * region.size[1] as usize)
        })
        .count()
}

fn main() {
    let input = parse_input(include_str!("../../data/day12.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
}
