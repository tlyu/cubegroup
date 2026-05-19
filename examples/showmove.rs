use std::env;
use std::error::Error;
use std::io;

use cubegroup::*;

fn one(s: &str) -> Result<(), Box<dyn Error>> {
    let t = s.parse::<Turns>()?;
    let c = Cube::default() * &t;
    println!("{} position: {}", &t, c);
    println!("{} cycles:   {}", &t, c.cycles());
    println!("{} position (Speffz): {}", &t, c.speffz());
    println!("{} cycles (Speffz):   {}", &t, c.cycles().speffz());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let iter: &mut dyn Iterator<Item=Result<String, _>> = if args.len() > 1 {
        &mut args.skip(1).map(|x| Ok(x))
    } else {
        eprintln!("Reading moves from standard input...");
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
