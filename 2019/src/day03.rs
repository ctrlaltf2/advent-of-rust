use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

type Wire = Vec<Point>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|ln| {
            let mut wire: Vec<Point> = Vec::new();
            let (mut x, mut y): (i32, i32) = (0, 0);
            for instr in ln.split(",").map(|s| s.trim()).filter(|s| s.len() > 0) {
                let (mut dx, mut dy): (i32, i32) = (0, 0);
                match instr.chars().next().unwrap() {
                    'U' => dy = 1,
                    'D' => dy = -1,
                    'L' => dx = -1,
                    'R' => dx = 1,
                    _ => println!("Invalid direction '{}'", instr.chars().nth(0).unwrap()),
                }

                let lim = instr[1..].parse().unwrap();

                for _ in 0..lim {
                    x += dx;
                    y += dy;

                    wire.push(Point { x: x, y: y });
                }
            }

            wire
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(wires: &Vec<Wire>) -> i32 {
    let a_wire: HashSet<&Point> = HashSet::from_iter(wires[0].iter());
    let b_wire: HashSet<&Point> = HashSet::from_iter(wires[1].iter());

    a_wire
        .intersection(&b_wire)
        .map(|cross| cross.x.abs() + cross.y.abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(wires: &Vec<Wire>) -> i32 {
    let a_wire: HashSet<&Point> = HashSet::from_iter(wires[0].iter());
    let b_wire: HashSet<&Point> = HashSet::from_iter(wires[1].iter());

    a_wire
        .intersection(&b_wire)
        .map(|cross| {
            let steps_a = 1 + wires[0]
                .iter()
                .position(|p| (p.x == cross.x) && (p.y == cross.y))
                .unwrap();
            let steps_b = 1 + wires[1]
                .iter()
                .position(|p| (p.x == cross.x) && (p.y == cross.y))
                .unwrap();

            (steps_a + steps_b) as i32
        })
        .min()
        .unwrap()
}
