type Number = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    distance: Number,
}

#[derive(Debug, Clone, Copy)]
struct Dial {
    position: Number,
}

impl Dial {
    fn new() -> Self {
        Self { position: 50 }
    }

    fn turn(&mut self, rotation: &Rotation) -> usize {
        if rotation.direction == Direction::Left {
            self.position = (100 - self.position) % 100;
        }
        let sum = self.position + rotation.distance;
        let times_at_zero = sum / 100;
        self.position = sum % 100;
        if rotation.direction == Direction::Left {
            self.position = (100 - self.position) % 100;
        }
        times_at_zero as usize
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| {
            let direction = match line.chars().next().unwrap() {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!(),
            };
            let distance = line[1..].parse().unwrap();
            Rotation {
                direction,
                distance,
            }
        })
        .collect()
}

fn solve_part1(input: &[Rotation]) -> usize {
    input
        .iter()
        .fold((Dial::new(), 0), |acc, x| {
            let mut dial = acc.0;
            dial.turn(x);
            (dial, acc.1 + if dial.position == 0 { 1 } else { 0 })
        })
        .1
}

fn solve_part2(input: &[Rotation]) -> usize {
    input
        .iter()
        .fold((Dial::new(), 0), |acc, x| {
            let mut dial = acc.0;
            let count = dial.turn(x);
            (dial, acc.1 + count)
        })
        .1
}

fn main() {
    let input = parse_input(include_str!("../../data/day01.txt"));
    let answer1 = solve_part1(&input);
    println!("The answer to part 1 is {answer1}");
    let answer2 = solve_part2(&input);
    println!("The answer to part 2 is {answer2}");
}
