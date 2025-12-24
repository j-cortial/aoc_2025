use std::{cmp::max, fmt::Display};

type Int = u64;

#[derive(Clone, Copy)]
struct Range {
    begin: Int,
    end: Int,
}

impl Range {
    fn contains(&self, i: Int) -> bool {
        i >= self.begin && i <= self.end
    }

    fn size(&self) -> usize {
        (self.end - self.begin) as usize + 1
    }
}

struct Input {
    ranges: Vec<Range>,
    ids: Vec<Int>,
}

fn parse_input(input: &str) -> Input {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    Input {
        ranges: ranges
            .lines()
            .map(|line| {
                let (begin, end) = line.split_once('-').unwrap();
                Range {
                    begin: begin.parse().unwrap(),
                    end: end.parse().unwrap(),
                }
            })
            .collect(),
        ids: ids.lines().map(|line| line.parse().unwrap()).collect(),
    }
}

fn solve_part1(input: &Input) -> impl Display {
    input
        .ids
        .iter()
        .filter(|&&id| input.ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn solve_part2(input: &Input) -> impl Display {
    let mut ranges = input.ranges.clone();

    ranges.sort_unstable_by_key(|range| range.begin);

    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i + 1].begin <= ranges[i].end {
            ranges[i].end = max(ranges[i].end, ranges[i + 1].end);
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }

    ranges.iter().map(|range| range.size()).sum::<usize>()
}

fn main() {
    let input = parse_input(include_str!("../../data/day05.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
