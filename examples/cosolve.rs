use std::ops::ControlFlow::{self, Break, Continue};

use cubegroup::*;
use cubegroup::dr::*;

// Expand base 3 digits as base 16 for better display
fn semi(co: u16) -> u32 {
    let mut out = 0u32;
    for i in 0u32..7 {
        out |= (((co / 3u16.pow(i)) % 3) as u32) << (4*i);
    }
    out
}

fn idfs(init_co: u16, depth: u8, prev_turn: Option<Turn>, table: &COMul, v: &mut Vec<Turn>) -> ControlFlow<(), ()> {
    if depth == 0 {
        if init_co == 0 {
            if let Some(t) = prev_turn {
                v.insert(0, t);
            }
            return Break(());
        } else {
            return Continue(());
        }
    }
    canonturns(prev_turn).into_iter().try_for_each(|t| {
        let co = table.mul(init_co, t);
        idfs(co, depth - 1, Some(t), table, v)
    }).map_break(|_| {
        if let Some(t) = prev_turn {
            v.insert(0, t)
        }
    })
}

fn main() {
    let table = COMul::new();
    for co in 0..NCO {
        let mut v = Vec::<Turn>::new();
        let r = (0..7).try_for_each(|d| {
            idfs(co, d, None, &table, &mut v)
        });
        match r {
            Break(_) => { println!("{:07x} {}", semi(co), Turns::from(&v[..]))},
            _ => { println!("{:07x} not found", semi(co)) },
        }
    }
}
