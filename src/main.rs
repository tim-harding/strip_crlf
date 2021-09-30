use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut iter = buffer.chars().peekable();
    let mut out = String::new();
    while let Some(next) = iter.next() {
        match next {
            '\r' => {
                if iter.peek() == Some(&'\n') {
                    let _ = iter.next();
                }
            }
            c => out.push(c),
        }
    }
    println!("{}", out);
    Ok(())
}
