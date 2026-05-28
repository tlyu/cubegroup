use std::ops::ControlFlow::{self, Break, Continue};

use cubegroup::*;
use cubegroup::dr::*;

fn init_table() -> Vec<Vec<u16>> {
    let mut v = Vec::new();
    for i in 0u16..NEO {
        let row: Vec<_> = allturns().into_iter().map(|t| {
            (Edges::from_eo(i) * t).eo()
        }).collect();
        v.push(row);
    }
    v
}

fn idfs(init_eo: u16, depth: u8, prev_turn: Option<Turn>, table: &Vec<Vec<u16>>, v: &mut Vec<Turn>) -> ControlFlow<(), ()> {
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
        let eo = table[init_eo as usize][t as usize];
        idfs(eo, depth - 1, Some(t), table, v)
    }).map_break(|_| {
        if let Some(t) = prev_turn {
            v.insert(0, t)
        }
    })
}

fn main() {
    let table = init_table();
    for eo in 0..NEO {
        let mut v = Vec::<Turn>::new();
        let r = (0..8).try_for_each(|d| {
            idfs(eo, d, None, &table, &mut v)
        });
        match r {
            Break(_) => { println!("{eo:011b} {}", Turns::from(&v[..]))},
            _ => { println!("{eo:011b} not found") },
        }
    }
}
