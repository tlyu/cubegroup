use std::time::Instant;
use itertools::Itertools;

use cubegroup::*;

fn main() {
    let mut v = Vec::new();
    let now = Instant::now();
    for i in 0u16..2048 {
        let row: Vec<_> = allturns().into_iter().map(|t| (Edges::set_eo(i) * t).eo()).collect();
        v.push(row);
    }
    let elapsed = now.elapsed();
    for row in v {
        println!("[{}],", row.iter().map(|x| format!("{:#05x}", x)).join(", "));
    }
    println!("{:2?}", elapsed);
}
