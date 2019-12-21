#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let mut bounds: Vec<i32> = input.split('-').map(|s| s.parse().unwrap()).collect();
    let mut good_passwords: i32 = 0;
    while bounds[0] < bounds[1] {
        let num_str = bounds[0].to_string();
        // Assume string is sorted and doesn't have a repeat
        let (mut is_sorted, mut has_repeat) = (true, false);
        for (i, c) in num_str.chars().enumerate() {
            if i == (num_str.len() - 1) {
                break;
            }

            let chunk = (c, num_str.chars().nth(i + 1).unwrap());

            // Compare decrease against the next char
            if !(chunk.0 <= chunk.1) {
                is_sorted = false;
                break;
            }

            // Check if there's a repeat
            if chunk.0 == chunk.1 {
                has_repeat = true;
            }
        }

        if is_sorted && has_repeat {
            good_passwords += 1;
        }

        bounds[0] += 1;
    }

    good_passwords
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let mut bounds: Vec<i32> = input.split('-').map(|s| s.parse().unwrap()).collect();
    let mut good_passwords: i32 = 0;
    while bounds[0] < bounds[1] {
        let num_str = bounds[0].to_string();
        // Assume string is sorted and doesn't have a repeat
        let (mut is_sorted, mut has_double) = (true, false);
        for (i, c) in num_str.chars().enumerate() {
            // End of string
            if i == (num_str.len() - 1) {
                break;
            }

            let chunk = (c, num_str.chars().nth(i + 1).unwrap());

            // Compare decrease against the next char
            if !(chunk.0 <= chunk.1) {
                is_sorted = false;
                break;
            }

            // Check if there's a repeat, then check the surroundings of that chunk
            if chunk.0 == chunk.1 {
                let padding = (num_str.chars().nth(i - 1), num_str.chars().nth(i + 2));
                // Use error handling magic to compare the padding of the chunk IF it exists to
                // check if just a two char repeat exists
                if (chunk.0 == padding.0.unwrap_or_default())
                    || (chunk.1 == padding.1.unwrap_or_default())
                {
                    continue;
                }

                // Passed checks so it must be a standalone double
                has_double = true;
            }
        }

        if is_sorted && has_double {
            good_passwords += 1;
        }

        bounds[0] += 1;
    }

    good_passwords
}
