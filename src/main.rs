use clap::Clap;
use std::io::{self, Read};

#[derive(Clap, Copy, Clone, PartialEq, Eq)]
struct Options {
    #[clap(short, long)]
    escaped: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options: Options = Options::parse();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    if options.escaped {
        escaped(&buffer);
    } else {
        unescaped(&buffer);
    }
    Ok(())
}

fn unescaped(stdin: &str) {
    let mut iter = stdin.chars().peekable();
    let mut out = String::new();
    while let Some(next) = iter.next() {
        match next {
            '\r' => {
                // Do nothing.
            }
            c => out.push(c),
        }
    }
    println!("{}", out);
}

fn escaped(stdin: &str) {
    let mut iter = stdin.chars().peekable();
    let mut out = String::new();
    while let Some(next) = iter.next() {
        match next {
            '\\' => {
                if iter.peek() == Some(&'r')
                    && iter.peek() == Some(&'\\')
                    && iter.peek() == Some(&'n')
                {
                    for _ in 0..3 {
                        let _ = iter.next();
                    }
                    out.push('\\');
                    out.push('n');
                }
            }
            c => out.push(c),
        }
    }
    println!("{}", out);
}
