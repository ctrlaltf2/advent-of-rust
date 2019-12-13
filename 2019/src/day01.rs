#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let masses = input.split('\n');
    let mut fuel_requirement: i32 = 0;
    for m in masses {
        if m.len() == 0 {
            continue;
        }

        let mass_num = m.trim().parse::<i32>().unwrap();

        let fuel_req = mass_num / 3 - 2;

        fuel_requirement += fuel_req
    }

    fuel_requirement
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let masses = input.split('\n');
    let mut fuel_requirement: i32 = 0;
    for m in masses {
        if m.len() == 0 {
            continue;
        }

        let mass_num = m.trim().parse::<i32>().unwrap();

        let mut fuel_req = mass_num / 3 - 2;

        while fuel_req > 0 {
            fuel_requirement += fuel_req;

            fuel_req /= 3;
            fuel_req -= 2;
        }
    }

    fuel_requirement
}
