use std::env;
use std::error::Error;
use std::io;
use cubegroup::*;

fn one(s: &str) -> Result<(), Box<dyn Error>> {
    let c = Cube::from_speffz(s).map_err(|_| "parse fail".to_string())?;
    println!("{}", c);
    println!("{}", c.cycles());
    println!("{}", c.cycles().speffz());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    if args.len() > 1 {
        for s in args.skip(1) {
            one(&s)?;
        }
    } else {
        eprintln!("Reading moves from standard input...");
        for s in io::stdin().lines() {
            one(&s?)?;
        }
    }
    Ok(())
}
