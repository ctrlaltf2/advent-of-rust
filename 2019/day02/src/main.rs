use std::env;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("Something happened :(");
    let mut part2: bool = false;
    for arg in env::args() {
        if arg == "--part2" {
            part2 = true;
        }
    }

    fn parse(s: &str) -> u32 {
        s.trim().parse::<u32>().unwrap()
    }

    let program: Vec<u32> = input.split(',').map(parse).collect();
    let mut memory = program.clone();

    let (mut noun, mut verb): (u32, u32) = (0, 0);
    if !part2 {
        // Part 1 inputs
        noun = 12;
        verb = 2;
    }

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

        if !part2 {
            println!("Value a position 0 after halting: {}", memory[0]);
            break;
        }

        if memory[0] == 19690720 {
            // Part 2 goal!
            println!("The pair of inputs to produce 19690720 was noun = {}, verb = {}. 100 * noun + verb = {}", noun, verb, 100 * noun + verb);
            break;
        }

        // Reset memory
        memory = program.clone();

        // Increment inputs
        // noun ranges from 0...u32::MAX, verb ranges from 0...noun
        if noun == verb {
            noun += 1;
            verb = 0;
        } else {
            verb += 1;
        }
    }
}
