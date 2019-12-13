#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<u32>) -> u32 {
    let mut memory = input.clone();

    memory[1] = 12;
    memory[2] = 2;

    let mut i: usize = 0;
    while memory[i] != 99 {
        let a = memory[i + 1] as usize;
        let b = memory[i + 2] as usize;
        let dst = memory[i + 3] as usize;

        match memory[i] {
            1 => memory[dst] = memory[a] + memory[b],
            2 => memory[dst] = memory[a] * memory[b],
            _ => println!("Warning: Invalid opcode encountered, i = {}", i),
        }

        i += 4;
    }

    memory[0]
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<u32>) -> u32 {
    let mut memory = input.clone();

    let (mut noun, mut verb): (u32, u32) = (0, 0);

    while noun != std::u32::MAX {
        memory[1] = noun;
        memory[2] = verb;
        let mut i: usize = 0;
        while memory[i] != 99 {
            let a = memory[i + 1] as usize;
            let b = memory[i + 2] as usize;
            let dst = memory[i + 3] as usize;

            match memory[i] {
                1 => memory[dst] = memory[a] + memory[b],
                2 => memory[dst] = memory[a] * memory[b],
                _ => println!("Warning: Invalid opcode encountered, i = {}", i),
            }

            i += 4;
        }

        if memory[0] == 19690720 {
            // Part 2 goal!
            break;
        } else {
            // Reset memory
            memory = input.clone();
        }

        // Increment inputs
        // noun ranges from 0...u32::MAX, verb ranges from 0...noun
        if noun == verb {
            noun += 1;
            verb = 0;
        } else {
            verb += 1;
        }
    }

    100 * noun + verb
}
