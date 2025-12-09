use std::ops::RangeInclusive;

type Number = u64;

type Input = Vec<RangeInclusive<Number>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

fn is_obviously_invalid(n: Number) -> bool {
    let s = format!("{n}");
    s.len() % 2 == 0 && s[0..s.len() / 2] == s[s.len() / 2..]
}

fn solve_part1(input: &Input) -> Number {
    input
        .iter()
        .flat_map(|range| range.clone().filter(|&n| is_obviously_invalid(n)))
        .sum()
}

fn is_invalid(n: Number) -> bool {
    let s = format!("{n}");
    (2..=s.len()).any(|i| {
        s.len() % i == 0
            && (1..i).all(|j| s[0..s.len() / i] == s[j * s.len() / i..(j + 1) * s.len() / i])
    })
}

fn solve_part2(input: &Input) -> Number {
    input
        .iter()
        .flat_map(|range| range.clone().filter(|&n| is_invalid(n)))
        .sum()
}

fn main() {
    let input = parse_input(include_str!("../../data/day02.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
