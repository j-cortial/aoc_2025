type Int = u64;

type Input = Vec<Vec<Int>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as Int)
                .collect()
        })
        .collect()
}

fn max_joltage(bank: &[Int], digits: usize) -> Int {
    let mut remainder = bank;
    let mut result = 0;
    for d in 1..=digits {
        let digit = remainder[..remainder.len() + d - digits]
            .iter()
            .max()
            .unwrap();
        let pos = remainder.iter().position(|x| x == digit).unwrap();
        result = result * 10 + digit;
        remainder = &remainder[pos + 1..];
    }
    result
}

fn solve_part1(input: &Input) -> Int {
    input.iter().map(|bank| max_joltage(bank, 2)).sum()
}

fn solve_part2(input: &Input) -> Int {
    input.iter().map(|bank| max_joltage(bank, 12)).sum()
}

fn main() {
    let input = parse_input(include_str!("../../data/day03.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
