use std::fmt::Display;
use std::ops::{Add, Mul};

type Int = u64;

type Op = fn(Int, Int) -> Int;

struct Input {
    operands: Vec<Vec<&'static str>>,
    operators: Vec<Op>,
}

fn parse_input(input: &'static str) -> Input {
    let lines: Vec<_> = input.lines().collect();

    let ops_with_starts: Vec<_> = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .collect();

    let operators = ops_with_starts
        .iter()
        .map(|(_, c)| match c {
            '+' => Add::add,
            '*' => Mul::mul,
            _ => panic!(),
        })
        .collect();

    let operands = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            let mut tokens: Vec<_> = ops_with_starts
                .windows(2)
                .map(|s| &line[s[0].0..s[1].0 - 1])
                .collect();

            tokens.push(&line[ops_with_starts.last().unwrap().0..lines[0].len()]);
            tokens
        })
        .collect();

    Input {
        operands,
        operators,
    }
}

fn solve_part1(input: &Input) -> impl Display {
    input
        .operators
        .iter()
        .enumerate()
        .map(|(i, op)| {
            input
                .operands
                .iter()
                .map(|n| n[i].trim().parse().unwrap())
                .reduce(op)
                .unwrap()
        })
        .sum::<Int>()
}

fn solve_part2(input: &Input) -> impl Display {
    input
        .operators
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let operand_count = input.operands[0][i].len();
            (0..operand_count)
                .map(|j| {
                    let mut res = 0;
                    for digit in input.operands.iter().map(|s| s[i].as_bytes()[j]) {
                        if digit != b' ' {
                            res *= 10;
                            res += (digit - b'0') as Int;
                        }
                    }
                    res
                })
                .reduce(op)
                .unwrap()
        })
        .sum::<Int>()
}

fn main() {
    let input = parse_input(include_str!("../../data/day06.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
