use std::io::{self, Write};
use std::ops::ControlFlow;
use std::time::Instant;

use cubegroup::*;

fn idfs<F, G, T>(depth: u8, prev_move: Option<Turn>, prev_cube: Cube, f: &mut F, g: &mut G) -> ControlFlow<T, ()>
    where F: FnMut(Cube) -> ControlFlow<T, ()>,
        G: FnMut(Turn) -> ()
{
    canonturns(prev_move).try_for_each(|t| {
        let cube = prev_cube * t;
        if depth > 0 {
            idfs(depth - 1, Some(t), cube, f, g)
        } else {
            f(cube)
        }.map_break(|x| { g(t); x })
    })
}

fn one(n: u8, init_cube: Cube) -> ControlFlow<u8, ()> {
    let mut count = 0u64;
    let mut v = Vec::<Turn>::new();
    let mut g = |t| { v.insert(0, t); };

    print!("trying depth {}... ", n + 1);
    io::stdout().flush().unwrap();
    let now = Instant::now();
    let r = idfs(n, None, init_cube, &mut |c| {
        count += 1;
        if c == Cube::default() { ControlFlow::Break(()) } else { ControlFlow::Continue(())}
    }, &mut g);
    let elapsed = now.elapsed();
    let rate = count as f32 / elapsed.as_micros() as f32;
    match r {
        ControlFlow::Break(_) => {
            println!("found! {elapsed:.2?} ({rate:.2}M/s)");
            println!("{}", Turns::from(&v[..]));
            ControlFlow::Break(n)
        },
        ControlFlow::Continue(_) => {
            println!("not found {elapsed:.2?} ({rate:.2}M/s)");
            ControlFlow::Continue(())
        }
    }
}

fn main() {
    // optimal J perm is 10 moves; this takes a while...
    // let init_cube = Cube::from_speffz("ACBDUVWX.ACBDRTJLUVWX").unwrap();
    // A perm
    let init_cube = Cube::from_speffz("BCADUVWX.ABCDRTJLUVWX").unwrap();
    println!("solving {}", init_cube.cycles());
    if let ControlFlow::Break(n) = (0..10u8).try_for_each(|n| {
            one(n, init_cube)
        })
    {
        println!("found at depth {}", n + 1);
    } else {
        println!("not found");
    }
}
