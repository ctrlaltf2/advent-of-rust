use std::collections::HashSet;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut part2 = false;
    for arg in &args {
        if arg == "part2" {
            part2 = true;
        }
    }

    let mut ln = String::new();
    let mut delta_freqs: Vec<i32> = Vec::new();
    loop {
        ln.clear();
        io::stdin().read_line(&mut ln)?;

        if ln.trim() == "" {
            break;
        }

        delta_freqs.push(ln.trim().parse::<i32>().unwrap());
    }

    if !part2 {
        let sum: i32 = delta_freqs.iter().sum();
        println!("Part 1: {}", sum);
        return Ok(());
    } else {
        let mut freq = 0;
        let mut set = HashSet::new();
        loop {
            for x in &delta_freqs {
                if !set.insert(freq) {
                    println!("Part 2: {}", freq);
                    return Ok(());
                }
                freq += x;
            }
        }
    }
}
