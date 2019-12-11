use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("./input").expect("Something happened :(");
    let first_wire = parse_wire(input.split('\n').nth(0).unwrap());
    let second_wire = parse_wire(input.split('\n').nth(1).unwrap());

    let mut min_distance: i32 = std::i32::MAX;
    for x in first_wire.intersection(&second_wire) {
        let distance = x.x.abs() + x.y.abs();
        if distance < min_distance {
            min_distance = distance;
        }
    }

    println!(
        "Manhatten distance of the closest intersection: {}",
        min_distance
    );
}

fn parse_wire(ln: &str) -> HashSet<Point> {
    let mut wire: HashSet<Point> = HashSet::new();
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

            wire.insert(Point { x: x, y: y });
        }
    }

    wire
}
