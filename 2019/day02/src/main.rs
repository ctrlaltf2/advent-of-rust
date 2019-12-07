use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("Something happened :(");
    let mut opcodes: Vec<i32> = Vec::new();
    for s in input.split(",") {
        if s.len() == 0 {
            continue;
        }
        opcodes.push(s.trim().parse::<i32>().unwrap());
    }

    // replace position 1 with the value 12 and replace position 2 with the value 2
    opcodes[1] = 12;
    opcodes[2] = 2;

    let mut halted: bool = false;
    let mut i: usize = 0;
    while !halted {
        match opcodes[i] {
            // add, a, b, dst
            1 => {
                let a = opcodes[i + 1] as usize;
                let b = opcodes[i + 2] as usize;
                let dst = opcodes[i + 3] as usize;
                opcodes[dst] = opcodes[a] + opcodes[b];
            }
            // mul, a, b, dst
            2 => {
                let a = opcodes[i + 1] as usize;
                let b = opcodes[i + 2] as usize;
                let dst = opcodes[i + 3] as usize;
                opcodes[dst] = opcodes[a] * opcodes[b];
            }
            99 => halted = true,
            _ => println!("Warning: Invalid opcode encountered, i = {}", i),
        }

        i += 4;
    }

    println!("{}", opcodes[0]);
}
