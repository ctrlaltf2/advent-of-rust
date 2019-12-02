use std::fs;

fn main() {
    let contents = fs::read_to_string("./input").expect("Something happened :(");
    let masses = contents.split('\n');
    let mut fuel_requirement: i32 = 0;
    for m in masses {
        if m.len() == 0 {
            continue;
        }
        let mass_num = m.trim().parse::<i32>().unwrap();

        fuel_requirement += mass_num / 3;
        fuel_requirement -= 2;
    }

    println!("Sum of the fuel requirements: {}", fuel_requirement);
}
