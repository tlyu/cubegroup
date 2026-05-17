use std::collections::{HashSet};
use std::hint::black_box;
use std::str::FromStr;

use cubegroup::*;

fn idfs<F: FnMut(Cube) -> ()>(level: u8, prev_move: Option<Turn>, prev_cube: Cube, f: &mut F) {
    for t in canonturns(prev_move) {
        let cube = prev_cube * t;
        if level != 0 {
            idfs(level - 1, Some(t), cube, f);
        } else {
            f(cube);
        }
    }
}

fn main() {
    use std::time::Instant;
    use Turn::*;
    let corners = corners_neon::Corners::default();
    println!("default corners: {corners}");
    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{}: {}", t, corners * t);
        println!("{} cycles: {}", t, (corners * t).cycles());
    }
    let s = "R";
    println!("{s}: {}", (corners * Turn::from_str(s).unwrap()).cycles());
    println!("sledge: {}", corners * &Turns::from(&[R3, F1, R1, F3]));
    println!("sledge cycles: {}", (corners * &Turns::from(&[R3, F1, R1, F3])).cycles());
    println!("double sledge cycles: {}", (corners * &Turns::from(&[R3, F1, R1, F3, R3, F1, R1, F3])).cycles());

    let edges = edges_neon::Edges::default();
    println!("default edges: {edges}");
    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{:?}: {}", t, edges * t);
        println!("{:?} cycles: {}", t, (edges * t).cycles());
    }
    println!("sledge: {}", edges * &Turns::from(&[R3, F1, R1, F3]));
    println!("sledge cycles: {}", (edges * &"R' F R F'".parse::<Turns>().unwrap()).cycles());

    let cube = Cube::default();
    println!("default cube: {cube}");
    println!("allturns: {:?}", allturns().collect::<Vec<_>>());
    for x in allturns() {
        println!("{:?}: {}", x, cube * x);
        println!("{}: {}", x, (cube * x).cycles());
    }
    println!("sledge: {}", cube.turns(&[R3, F1, R1, F3]));
    println!("sledge cycles: {}", cube.turns(&[R3, F1, R1, F3]).cycles());

    let s = Cube::default().turns(&[R1, U1]);
    println!("R U: {}", s.cycles());
    println!("F R U R' U' F': {}", cube.turns(&[F1, R1, U1, R3, U3, F3]).cycles());
    println!("F R U R' U' F' U: {}", cube.turns(&[F1, R1, U1, R3, U3, F3, U1]).cycles());

    println!("{}", "R U R' U'".parse::<Turns>().unwrap());
    let t = "F    R U R' U' F'".parse::<Turns>().unwrap();
    println!("{t}: {}", cube * &t);
    println!("{t}: {}", (cube * &t).cycles());

    let s = cube * &("R U2 D' B D'".parse::<Turns>().unwrap());
    let mut x = cube;
    let max = if cfg!(debug_assertions) {
        1u64<<27
    } else {
        1u64 << 31
    };
    println!("{} turns...", max);
    let now = Instant::now();
    for _ in 0..max {
        x = x * s;
    }
    black_box(x);
    let elapsed = now.elapsed();
    let rate: f32 = max as f32 / elapsed.as_micros() as f32;
    println!("{} turns {:.2?} {:.2}M/s", max, elapsed, rate);

    println!("bare DFS...");
    let max = if cfg!(debug_assertions) {
        7
    } else {
        9
    };
    let mut total = 0u64;
    for i in 0..max {
        let now = Instant::now();
        let mut count = 0u64;
        idfs(i, None, cube, &mut |_| { count += 1; });
        total += count;
        let elapsed = now.elapsed();
        let rate = total as f32 / elapsed.as_micros() as f32;
        println!("level {} count {} {:.2?} ({:.2}M/s)", i+1, count, elapsed, rate);
    }

    #[cfg(debug_assertions)]
    let max = 6;
    #[cfg(not(debug_assertions))]
    let max = 7;
    let mut total = 0u64;
    let mut hash = HashSet::<Cube>::new();
    hash.insert(cube);
    let start = Instant::now();
    println!("counting unique positions...");
    for i in 0..max {
        let now = Instant::now();
        let mut count = 0u64;
        idfs(i, None, cube, &mut |x| { if hash.insert(x) { count += 1; }});
        total += count;
        let elapsed = now.elapsed();
        let rate = total as f32 / elapsed.as_micros() as f32;
        println!("level {} count {} {:.2?} ({:.2}M/s)", i+1, count, elapsed, rate)
    }
    println!("total elapsed {:.2?}", start.elapsed());
}
