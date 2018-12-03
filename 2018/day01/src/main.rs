use std::io;

fn main() -> io::Result<()> {
    let mut ln = String::new();
    let mut sum = 0;
    loop {
        ln.clear();
        io::stdin().read_line(&mut ln)?;
        if ln.trim() == "" {
            break;
        } else {
            sum += ln.trim().parse::<i32>().unwrap();
        }
    }
    println!("'{}'", sum);
    Ok(())
}
