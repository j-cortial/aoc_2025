use std::{collections::HashSet, fmt::Display};

type Coord = i32;
type Loc = [Coord; 2];

type Input = HashSet<Loc>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.bytes().enumerate().filter_map(move |(j, tile)| {
                if tile == b'@' {
                    Some([i as Coord, j as Coord])
                } else {
                    None
                }
            })
        })
        .collect()
}

static MOVES: [Loc; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

fn add(a: Loc, b: Loc) -> Loc {
    [a[0] + b[0], a[1] + b[1]]
}

fn accessible(input: &Input, loc: Loc) -> bool {
    MOVES
        .iter()
        .filter(|&&m| input.contains(&add(m, loc)))
        .count()
        < 4
}

fn solve_part1(input: &Input) -> impl Display {
    input.iter().filter(|&&loc| accessible(input, loc)).count()
}

fn solve_part2(input: &Input) -> impl Display {
    let mut state = input.clone();
    while let Some(&loc) = state.iter().find(|&&loc| accessible(&state, loc)) {
        state.remove(&loc);
    }
    input.len() - state.len()
}

fn main() {
    let input = parse_input(include_str!("../../data/day04.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
