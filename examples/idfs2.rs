// Double-ended IDFS

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io::{self, Write};
use std::ops::ControlFlow::{self, Break, Continue};
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

fn one(n: u8, init_cube: Cube, h: &mut HashMap<Cube, u8>) -> ControlFlow<(u8, Vec<Turn>, Cube), ()>
{
    let mut count = 0u64;
    let mut v = Vec::<Turn>::new();
    let mut g = |t| { v.insert(0, t); };
    let d = if n & 1 == 1 { n / 2 } else { (n / 2) - 1 };

    print!("trying depth {}({})... ", n, d);
    io::stdout().flush().unwrap();
    let now = Instant::now();
    let cube = if n & 1 == 0 { Cube::default() } else { init_cube };
    let r = idfs(d, None, cube, &mut |c| {
        count += 1;
        match h.entry(c) {
            Occupied(v) => {
                let v = *v.get();
                if v == 0 || v & 1 != n & 1 {
                    Break((v, c))
                } else {
                    Continue(())
                }
            },
            Vacant(v) => {
                v.insert(n);
                Continue(())
            },
        }
    }, &mut g);
    let elapsed = now.elapsed();
    let rate = count as f32 / elapsed.as_micros() as f32;
    match r {
        Break((depth, c)) => {
            println!("found! {count} in {elapsed:.2?} ({rate:.2}M/s)");
            println!("{}", Turns::from(&v[..]));
            Break((depth, v, c))
        },
        Continue(_) => {
            println!("not found {count} in {elapsed:.2?} ({rate:.2}M/s)");
            Continue(())
        }
    }
}

fn main() {
    // T perm
    // let init_cube = Cube::from_speffz("ACBDUVWX.ADCBRTJLUVWX").unwrap();
    // N perm
    let init_cube = Cube::from_speffz("ADCBUVWX.ADCBRTJLUVWX").unwrap();
    // J perm
    // let init_cube = Cube::from_speffz("ACBDUVWX.ACBDRTJLUVWX").unwrap();
    // A perm
    // let init_cube = Cube::from_speffz("BCADUVWX.ABCDRTJLUVWX").unwrap();
    // let init_cube = Cube::default() * "R".parse::<Turns>().unwrap();
    // Per special
    // let init_cube = Cube::from_speffz("ADCVUBWX.ABCDRTJLUVWX").unwrap();
    // Z perm
    // let init_cube = Cube::from_speffz("ABCDUVWX.DCBARTJLUVWX").unwrap();
    let mut hash = HashMap::<Cube, u8>::new();
    hash.insert(Cube::default(), 0);

    println!("solving {}", init_cube.cycles());
    if let Break((n, v, c)) = (1..=14u8).try_for_each(|n| {
            one(n, init_cube, &mut hash)
    })
    {
        let t1 = Turns::from(&v[..]);
        if n == 0 {
            println!("solved!");
            return
        } else {
            println!("matched at depth {n}");
        }
        let mut v = Vec::<Turn>::new();
        let mut count = 0u64;
        let start_cube = if n & 1 == 0 { c } else { init_cube };
        let match_cube = if n & 1 == 0 { Cube::default() } else { c };
        let d = if n & 1 == 1 { n/2 } else { n/2 - 1 };
        // println!("d={} c={} start={} match={}", d, c.cycles(), start_cube.cycles(), match_cube.cycles());
        let now = Instant::now();
        let r = idfs(d, None, start_cube, &mut |c| {
            count += 1;
            if c == match_cube { Break(()) } else { Continue(()) }
        }, &mut |t| {
            v.insert(0, t);
        });
        let elapsed = now.elapsed();
        let rate = count as f32 / elapsed.as_micros() as f32;
        println!("finish {count} in {elapsed:.2?} ({rate:.2}M/s)");
        match r {
            Break(_) => {
                let t2 = Turns::from(&v[..]);
                if n & 1 == 0 {
                    println!("{t1} {t2}");
                } else {
                    println!("{t2} {}", !t1);
                }
            },
            Continue(_) => { panic!("not found after match!"); },
        }
    } else {
        println!("not found");
    }
}
