use std::ops::ControlFlow::{self, Break, Continue};

use cubegroup::*;
use cubegroup::dr::*;

fn idfs(init_eo: u16, depth: u8, prev_turn: Option<Turn>, v: &mut Vec<Turn>) -> ControlFlow<(), ()> {
    if depth == 0 {
        if init_eo == 0 {
            if let Some(t) = prev_turn {
                v.insert(0, t);
            }
            return Break(());
        } else {
            return Continue(());
        }
    }
    canonturns(prev_turn).into_iter().try_for_each(|t| {
        let eo = eo_mul(init_eo, t);
        idfs(eo, depth - 1, Some(t), v)
    }).map_break(|_| {
        if let Some(t) = prev_turn {
            v.insert(0, t)
        }
    })
}

fn main() {
    // let m = EOMul::new();
    init_eo_mul();
    for eo in 0..NEO {
        let mut v = Vec::<Turn>::new();
        let r = (0..8).try_for_each(|d| {
            idfs(eo, d, None, &mut v)
        });
        match r {
            Break(_) => { println!("{eo:011b} {}", Turns::from(&v[..]))},
            _ => { println!("{eo:011b} not found") },
        }
    }
}
