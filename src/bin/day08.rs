use std::{cmp::Ordering, collections::BinaryHeap, fmt::Display};

type Coord = i32;
type Loc = [Coord; 3];
type SquaredDistance = u64;

type Input = Vec<Loc>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split(',');
            [
                tokens.next().unwrap().parse().unwrap(),
                tokens.next().unwrap().parse().unwrap(),
                tokens.next().unwrap().parse().unwrap(),
            ]
        })
        .collect()
}

fn squared_distance(a: &Loc, b: &Loc) -> SquaredDistance {
    a.iter()
        .zip(b.iter())
        .map(|(u, v)| (u - v).abs() as SquaredDistance * (u - v).abs() as SquaredDistance)
        .sum()
}

#[derive(PartialEq, Eq, Ord)]
struct Candidate {
    locations: [Loc; 2],
    distance: SquaredDistance,
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

fn create_candidates(locations: &[Loc]) -> BinaryHeap<Candidate> {
    locations
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            locations[i + 1..].iter().map(|b| Candidate {
                locations: [*a.min(b), *a.max(b)],
                distance: squared_distance(a, b),
            })
        })
        .collect()
}

fn create_initial_circuits(locations: &[Loc]) -> Vec<Vec<Loc>> {
    locations.iter().map(|&v| vec![v]).collect()
}

fn create_link(circuits: &mut Vec<Vec<Loc>>, candidates: &mut BinaryHeap<Candidate>) -> [Loc; 2] {
    let linked_locations = candidates.pop().unwrap().locations;
    let old_circuit_index = circuits
        .iter()
        .position(|circuit| circuit.contains(&linked_locations[0]))
        .unwrap();
    if !circuits[old_circuit_index].contains(&linked_locations[1]) {
        let mut old_circuit = circuits.remove(old_circuit_index);
        let new_circuit = circuits
            .iter_mut()
            .find(|circuit| circuit.contains(&linked_locations[1]))
            .unwrap();
        new_circuit.append(&mut old_circuit);
    }
    linked_locations
}

fn solve_part1(input: &Input) -> impl Display {
    let mut candidates = create_candidates(input);
    let mut circuits = create_initial_circuits(input);

    for _ in 0..1000 {
        create_link(&mut circuits, &mut candidates);
    }

    let mut circuit_sizes: Vec<_> = circuits.into_iter().map(|c| c.len()).collect();
    circuit_sizes.select_nth_unstable_by(3, |a, b| usize::cmp(b, a));
    circuit_sizes.iter().take(3).product::<usize>()
}

fn solve_part2(input: &Input) -> impl Display {
    let mut candidates = create_candidates(input);
    let mut circuits = create_initial_circuits(input);

    let last_link = loop {
        let linked_locations = create_link(&mut circuits, &mut candidates);
        if circuits.len() == 1 {
            break linked_locations;
        }
    };

    last_link[0][0] as SquaredDistance * last_link[1][0] as SquaredDistance
}

fn main() {
    let input = parse_input(include_str!("../../data/day08.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
