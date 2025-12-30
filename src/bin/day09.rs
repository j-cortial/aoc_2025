use std::fmt::Display;

type Coord = i32;
type Loc = [Coord; 2];
type Input = Vec<Loc>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect()
}

type Volume = u64;

fn enclosed_rectangle_volume(a: &Loc, b: &Loc) -> Volume {
    ((a[0] - b[0]).abs() + 1) as Volume * ((a[1] - b[1]).abs() + 1) as Volume
}

fn solve_part1(input: &Input) -> impl Display {
    input
        .iter()
        .flat_map(|a| input.iter().map(|b| enclosed_rectangle_volume(a, b)))
        .max()
        .unwrap()
}

#[derive(Clone, Copy)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

fn compute_rotation(nodes: &[Loc]) -> Rotation {
    let y_max = nodes.iter().map(|loc| loc[1]).max().unwrap();
    let y_max_index = nodes.iter().position(|loc| loc[1] == y_max).unwrap();
    let xs = [
        nodes[y_max_index][0],
        nodes[(y_max_index + 1) % nodes.len()][0],
    ];
    if xs[0] < xs[1] {
        Rotation::Clockwise
    } else {
        Rotation::CounterClockwise
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Segment {
    direction: Direction,
    coordinate: Coord,
    low: Coord,
    high: Coord,
}

fn compute_segment(rotation: Rotation, begin: &Loc, end: &Loc) -> Segment {
    if begin[0] == end[0] {
        let coordinate = begin[0];
        if begin[1] < end[1] {
            let direction = match rotation {
                Rotation::Clockwise => Direction::Down,
                Rotation::CounterClockwise => Direction::Up,
            };
            Segment {
                direction,
                coordinate,
                low: begin[1],
                high: end[1],
            }
        } else {
            let direction = match rotation {
                Rotation::Clockwise => Direction::Up,
                Rotation::CounterClockwise => Direction::Down,
            };
            Segment {
                direction,
                coordinate,
                low: end[1],
                high: begin[1],
            }
        }
    } else {
        let coordinate = begin[1];
        if begin[0] < end[0] {
            let direction = match rotation {
                Rotation::Clockwise => Direction::Left,
                Rotation::CounterClockwise => Direction::Right,
            };
            Segment {
                direction,
                coordinate,
                low: begin[0],
                high: end[0],
            }
        } else {
            let direction = match rotation {
                Rotation::Clockwise => Direction::Right,
                Rotation::CounterClockwise => Direction::Left,
            };
            Segment {
                direction,
                coordinate,
                low: end[0],
                high: begin[0],
            }
        }
    }
}

fn compute_segments(nodes: &[Loc]) -> Vec<Segment> {
    let rotation = compute_rotation(nodes);
    let mut result: Vec<Segment> = nodes
        .windows(2)
        .map(|ns| compute_segment(rotation, &ns[0], &ns[1]))
        .collect();
    result.push(compute_segment(
        rotation,
        &nodes[nodes.len() - 1],
        &nodes[0],
    ));
    result
}

fn is_compatible_rectangle(
    segment: &Segment,
    x_min: Coord,
    x_max: Coord,
    y_min: Coord,
    y_max: Coord,
) -> bool {
    match segment.direction {
        Direction::Up => {
            let ok_x = segment.coordinate >= x_max || segment.coordinate < x_min;
            let ok_y = segment.low >= y_max || segment.high <= y_min;
            ok_x || ok_y
        }
        Direction::Down => {
            let ok_x = segment.coordinate > x_max || segment.coordinate <= x_min;
            let ok_y = segment.low >= y_max || segment.high <= y_min;
            ok_x || ok_y
        }
        Direction::Right => {
            let ok_x = segment.low >= x_max || segment.high <= x_min;
            let ok_y = segment.coordinate > y_max || segment.coordinate <= y_min;
            ok_x || ok_y
        }
        Direction::Left => {
            let ok_x = segment.low >= x_max || segment.high <= x_min;
            let ok_y = segment.coordinate >= y_max || segment.coordinate < y_min;
            ok_x || ok_y
        }
    }
}

fn solve_part2(input: &Input) -> impl Display {
    let segments = compute_segments(input);
    input
        .iter()
        .flat_map(|a| {
            input.iter().filter_map(|b| {
                if segments.iter().all(|segment| {
                    is_compatible_rectangle(
                        segment,
                        a[0].min(b[0]),
                        a[0].max(b[0]),
                        a[1].min(b[1]),
                        a[1].max(b[1]),
                    )
                }) {
                    Some(enclosed_rectangle_volume(a, b))
                } else {
                    None
                }
            })
        })
        .max()
        .unwrap()
}

fn main() {
    let input = parse_input(include_str!("../../data/day09.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
