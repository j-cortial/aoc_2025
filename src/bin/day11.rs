use std::{collections::HashMap, convert::identity, fmt::Display};

type Device = &'static str;

type Input = HashMap<Device, Vec<Device>>;

fn parse_input(input: &'static str) -> Input {
    input
        .lines()
        .map(|line| {
            let (device, outputs) = line.split_once(':').unwrap();
            let outputs = outputs.trim().split_whitespace().collect();
            (device, outputs)
        })
        .collect()
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Status {
    node: Device,
    visited_steps: Vec<bool>,
}

impl Status {
    fn new(root: Device, visited_step_count: usize) -> Self {
        Status {
            node: root,
            visited_steps: vec![false; visited_step_count],
        }
    }
}

fn solve_dfs(
    status: Status,
    target: Device,
    steps: &[Device],
    graph: &HashMap<Device, Vec<Device>>,
    memory: &mut HashMap<Status, usize>,
) -> usize {
    if let Some(&result) = memory.get(&status) {
        result
    } else {
        let result = solve_dfs_impl(status.clone(), target, steps, graph, memory);
        memory.insert(status, result);
        result
    }
}

fn solve_dfs_impl(
    status: Status,
    target: Device,
    steps: &[Device],
    graph: &HashMap<Device, Vec<Device>>,
    memory: &mut HashMap<Status, usize>,
) -> usize {
    if status.node == target {
        if status.visited_steps.iter().copied().all(identity) {
            1
        } else {
            0
        }
    } else {
        let Status {
            node,
            mut visited_steps,
        } = status;
        if let Some(index) = steps.iter().position(|&step| step == node) {
            visited_steps[index] = true;
        }
        graph
            .get(node)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|next| {
                solve_dfs(
                    Status {
                        node: next,
                        visited_steps: visited_steps.clone(),
                    },
                    target,
                    steps,
                    graph,
                    memory,
                )
            })
            .sum::<usize>()
    }
}

fn solve_part1(input: &Input) -> impl Display {
    let mut memory: HashMap<Status, usize> = HashMap::new();
    solve_dfs(Status::new("you", 0), "out", &vec![], input, &mut memory)
}

fn solve_part2(input: &Input) -> impl Display {
    let mut memory: HashMap<Status, usize> = HashMap::new();
    solve_dfs(
        Status::new("svr", 2),
        "out",
        &vec!["dac", "fft"],
        input,
        &mut memory,
    )
}

fn main() {
    let input = parse_input(include_str!("../../data/day11.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
