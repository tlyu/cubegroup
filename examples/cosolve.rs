use std::ops::ControlFlow::{self, Break, Continue};

use cubegroup::*;

// Expand base 3 digits as base 16 for better display
fn semi(co: u16) -> u32 {
    let mut out = 0u32;
    for i in 0u32..7 {
        out |= (((co / 3u16.pow(i)) % 3) as u32) << (4*i);
    }
    out
}

fn init_table() -> Vec<Vec<u16>> {
    let mut v = Vec::new();
    for i in 0u16..2187 {
        let row: Vec<_> = allturns().into_iter().map(|t| {
            (Corners::set_co(i) * t).co()
        }).collect();
        v.push(row);
    }
    v
}

fn idfs(init_co: u16, depth: u8, prev_turn: Option<Turn>, table: &Vec<Vec<u16>>, v: &mut Vec<Turn>) -> ControlFlow<(), ()> {
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
        let co = table[init_co as usize][t as usize];
        idfs(co, depth - 1, Some(t), table, v)
    }).map_break(|_| {
        if let Some(t) = prev_turn {
            v.insert(0, t)
        }
    })
}

fn main() {
    let table = init_table();
    for co in 0..2187 {
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
