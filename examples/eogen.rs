use std::time::Instant;
use itertools::Itertools;

use cubegroup::*;

fn main() {
    let mut v = Vec::new();
    let now = Instant::now();
    for i in 0u16..2048 {
        let parity = ((i.count_ones() & 1) << 11) as u16;
        let eo = i | parity;
        let row: Vec<_> = allturns().into_iter().map(|t| (Edges::set_eo(eo) * t).eo()).collect();
        v.push(row);
    }
    let elapsed = now.elapsed();
    for row in v {
        println!("[{}],", row.iter().map(|x| format!("{:#05x}", x)).join(", "));
    }
    println!("{:2?}", elapsed);
}
