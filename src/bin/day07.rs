use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    fmt::Display,
};

struct Input {
    start: usize,
    splitters: Vec<HashSet<usize>>,
}

fn parse_input(input: &str) -> Input {
    let start = input
        .lines()
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .position(|&c| c == b'S')
        .unwrap();

    let splitters = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(i, &c)| if c == b'^' { Some(i) } else { None })
                .collect()
        })
        .collect();

    Input { start, splitters }
}

fn solve_part1(input: &Input) -> impl Display {
    let mut beams = HashSet::<usize>::new();
    beams.insert(input.start);

    let mut result: usize = 0;
    for row in &input.splitters {
        let mut new_beams = HashSet::new();
        for beam in beams {
            if row.contains(&beam) {
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
                result += 1;
            } else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }
    result
}

fn increment(map: &mut HashMap<usize, usize>, key: usize, value: usize) {
    match map.entry(key) {
        Entry::Occupied(mut occupied_entry) => {
            *occupied_entry.get_mut() += value;
        }
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(value);
        }
    }
}

fn solve_part2(input: &Input) -> impl Display {
    let mut beams = HashMap::<usize, usize>::new();
    beams.insert(input.start, 1);

    for row in &input.splitters {
        let mut new_beams = HashMap::new();
        for beam in beams {
            if row.contains(&beam.0) {
                increment(&mut new_beams, beam.0 - 1, beam.1);
                increment(&mut new_beams, beam.0 + 1, beam.1);
            } else {
                increment(&mut new_beams, beam.0, beam.1);
            }
        }
        beams = new_beams;
    }

    beams.into_values().sum::<usize>()
}

fn main() {
    let input = parse_input(include_str!("../../data/day07.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
