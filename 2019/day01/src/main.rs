use std::env;
use std::fs;

fn main() {
    let mut part2: bool = false;
    for arg in env::args() {
        if arg == "--part2" {
            part2 = true;
        }
    }

    let contents = fs::read_to_string("./input").expect("Something happened :(");
    let masses = contents.split('\n');
    let mut fuel_requirement: i32 = 0;
    for m in masses {
        if m.len() == 0 {
            continue;
        }
        let mass_num = m.trim().parse::<i32>().unwrap();

        let mut fuel_req = mass_num / 3 - 2;

        if part2 {
            while fuel_req > 0 {
                fuel_requirement += fuel_req;

                fuel_req /= 3;
                fuel_req -= 2;
            }
        } else {
            fuel_requirement += fuel_req
        }
    }

    println!("Sum of the fuel requirements: {}", fuel_requirement);
}
