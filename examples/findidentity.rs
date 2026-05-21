use std::ops::ControlFlow;
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

fn one(n: u8) -> ControlFlow<u8, ()> {
    let mut v = Vec::<Turn>::new();
    let mut g = |t| { v.insert(0, t); };
    print!("trying depth {}... ", n + 1);
    let r = idfs(n, None, Cube::default(), &mut |c| {
        if c == Cube::default() { ControlFlow::Break(()) } else { ControlFlow::Continue(())}
    }, &mut g);
    match r {
        ControlFlow::Break(_) => {
            println!("found! {}", Turns::from(&v[..]));
            ControlFlow::Break(n)
        },
        ControlFlow::Continue(_) => {
            println!("not found");
            ControlFlow::Continue(())
        }
    }
}

fn main() {
    println!("Searching for a non-trivial identity...");
    if let ControlFlow::Break(n) = (0..8u8).try_for_each(|n| {
            one(n)
        })
    {
        println!("found at depth {}", n + 1);
    } else {
        println!("not found");
    }
}
