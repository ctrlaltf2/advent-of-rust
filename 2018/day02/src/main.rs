use std::env;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut part2 = false;
    for arg in &args {
        if arg == "part2" {
            part2 = true;
        }
    }

    let (mut twos, mut threes) = (0, 0);
    loop {
        let mut ln = String::new();
        io::stdin().read_line(&mut ln)?;
        
        ln = ln.trim().to_string();

        if ln == "" {
            break;
        }

        let mut chars: HashMap<char, u32> = HashMap::new();
        
        for ch in ln.chars() {
            let count = chars.entry(ch).or_insert(0);
            *count += 1;
        }

        let (mut seen_two, mut seen_three) = (false, false);

        for (_, count) in chars {
            if count == 2 && !seen_two {
                twos += 1;
                seen_two = true;
            }

            if count == 3 && !seen_three {
                threes += 1;
                seen_three = true;
            }
        }
    }

    println!("Part 1: {}", threes * twos);

    Ok(())
}
