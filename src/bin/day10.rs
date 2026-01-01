use std::{fmt::Display, usize};

type Joltage = u16;

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    requirements: Vec<Joltage>,
}

type Input = Vec<Machine>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let groups: Vec<_> = line.split_whitespace().collect();

            let lights = groups[0]
                .trim_matches(['[', ']'])
                .bytes()
                .map(|b| b == b'#')
                .collect();
            let buttons = groups[1..groups.len() - 1]
                .iter()
                .map(|&group| {
                    group
                        .trim_matches(['(', ')'])
                        .split(',')
                        .map(|token| token.parse().unwrap())
                        .collect()
                })
                .collect();
            let requirements = groups[groups.len() - 1]
                .trim_matches(['{', '}'])
                .split(',')
                .map(|token| token.parse().unwrap())
                .collect();

            Machine {
                lights,
                buttons,
                requirements,
            }
        })
        .collect()
}

type Binary = u16;

fn bools_to_binary_impl(bools: impl Iterator<Item = bool>) -> Binary {
    bools.fold(0, |acc, x| 2 * acc + if x { 1 } else { 0 })
}

fn bools_to_binary(bools: &[bool]) -> Binary {
    bools_to_binary_impl(bools.iter().copied().rev())
}

fn indices_to_binary(indices: &[usize]) -> Binary {
    indices.iter().map(|&i| 1 << i).sum::<Binary>()
}

fn contains(combination: Binary, index: usize) -> bool {
    (combination >> index) % 2 == 1
}

fn compute_valid_combinations(buttons: &[Binary], lights: Binary) -> Vec<Binary> {
    (0..(1 << buttons.len()))
        .filter(|&combination| {
            let actual = (0..buttons.len())
                .filter(|&b| contains(combination, b))
                .fold(0, |acc, x| acc ^ buttons[x]);
            actual == lights
        })
        .collect()
}

fn solve_part1(input: &Input) -> impl Display {
    input
        .iter()
        .map(|machine| {
            compute_valid_combinations(
                &machine
                    .buttons
                    .iter()
                    .map(|b| indices_to_binary(b))
                    .collect::<Vec<_>>(),
                bools_to_binary(&machine.lights),
            )
            .into_iter()
            .map(|candidate| candidate.count_ones() as usize)
            .min()
            .unwrap()
        })
        .sum::<usize>()
}

fn compute_lights(requirements: &[Joltage]) -> Binary {
    bools_to_binary_impl(requirements.iter().rev().map(|&j| j % 2 == 1))
}

fn compute_delta(
    requirements: &[Joltage],
    buttons: &[Binary],
    combination: Binary,
) -> Vec<Joltage> {
    (0..buttons.len())
        .filter(|&b| contains(combination, b))
        .fold(vec![0; requirements.len()], |mut acc, b| {
            for index in 0..acc.len() {
                if contains(buttons[b], index) {
                    acc[index] += 1;
                }
            }
            acc
        })
}

fn compute_new_requirements(
    requirements: &[Joltage],
    buttons: &[Binary],
    combination: Binary,
) -> Option<Vec<Joltage>> {
    let delta = compute_delta(requirements, buttons, combination);
    if delta.iter().zip(requirements.iter()).any(|(d, r)| d > r) {
        None
    } else {
        Some(
            delta
                .iter()
                .zip(requirements.iter())
                .map(|(&d, &r)| r - d)
                .collect(),
        )
    }
}

fn solve_joltage_impl(requirements: &[Joltage], buttons: &[Binary]) -> Option<usize> {
    if requirements.iter().all(|&r| r == 0) {
        return Some(0);
    }
    let combinations = compute_valid_combinations(&buttons, compute_lights(&requirements));
    combinations
        .into_iter()
        .filter_map(|combination| {
            compute_new_requirements(&requirements, buttons, combination).and_then(|nr| {
                solve_joltage_impl(&nr.iter().map(|v| v / 2).collect::<Vec<_>>(), buttons)
                    .map(|rec| 2 * rec + combination.count_ones() as usize)
            })
        })
        .min()
}

fn solve_joltage(machine: &Machine) -> usize {
    solve_joltage_impl(
        &machine.requirements,
        &machine
            .buttons
            .iter()
            .map(|b| indices_to_binary(b))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn solve_part2(input: &Input) -> impl Display {
    input.iter().map(solve_joltage).sum::<usize>()
}

fn main() {
    let input = parse_input(include_str!("../../data/day10.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
