use std::env;
use std::error::Error;
use std::io;
use cubegroup::*;

fn one(s: &str) -> Result<(), Box<dyn Error>> {
    let c = Cube::from_speffz(s)?;
    println!("{}", c);
    println!("{}", c.cycles());
    println!("{}", c.cycles().speffz());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let iter: &mut dyn Iterator<Item=Result<String, _>> = if args.len() > 1 {
        &mut args.skip(1).map(|x| Ok(x))
    } else {
        eprintln!("Reading positions from standard input...");
        &mut io::stdin().lines()
    };
    for s in iter {
        match one(&s?) {
            Err(e) => eprintln!("{}", e),
            Ok(_) => ()
        }
    }
    Ok(())
}
