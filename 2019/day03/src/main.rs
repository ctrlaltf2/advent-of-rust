use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("./input").expect("Something happened :(");
    let a_wire = parse_wire(input.split('\n').nth(0).unwrap());
    let b_wire = parse_wire(input.split('\n').nth(1).unwrap());

    let a_set: HashSet<&Point> = HashSet::from_iter(a_wire.iter());
    let b_set: HashSet<&Point> = HashSet::from_iter(b_wire.iter());

    let mut min_distance: i32 = std::i32::MAX;
    for x in a_set.intersection(&b_set) {
        let distance = x.x.abs() + x.y.abs();
        if distance < min_distance {
            min_distance = distance;
        }
    }

    let mut min_combined_steps: i32 = std::i32::MAX;
    for &x in a_set.intersection(&b_set) {
        let steps_a = a_wire.iter().position(|p| p == x).unwrap() + 1;
        let steps_b = b_wire.iter().position(|p| p == x).unwrap() + 1;
        if ((steps_a + steps_b) as i32) < min_combined_steps {
            min_combined_steps = (steps_a + steps_b) as i32;
        }
    }

    println!(
        "Manhatten distance of the closest intersection: {}",
        min_distance
    );

    println!(
        "Minimum combined steps to an intersection: {}",
        min_combined_steps
    );
}

fn parse_wire(ln: &str) -> Vec<Point> {
    let mut wire: Vec<Point> = Vec::new();
    let (mut x, mut y): (i32, i32) = (0, 0);
    for instr in ln.split(",") {
        if instr.trim().len() == 0 {
            continue;
        }
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
}
