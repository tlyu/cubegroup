use std::cell::RefCell;
use std::collections::BTreeMap;
// use std::collections::btree_map::Entry;
use std::str::FromStr;

use cube_foo::*;

fn do_level(level: u8, max_level: u8, prev_move: Turn, prev_cube: Cube, hash: &RefCell<BTreeMap<u128, u8>>) {
    let iter = if level < 2 { Turn::allturns() } else { prev_move.into_iter() };
    for t in iter {
        let cube = prev_cube * t;
        {
            let mut muthash = hash.borrow_mut();
            muthash.entry(cube.pack()).and_modify(|x| *x = level.min(*x)).or_insert(level);
        }
        if level != max_level {
            do_level(level + 1, max_level, t, cube, hash);
        }
    }
    // println!("level {:width$} move {}", level, prev_move, width = (level as usize));
}

fn dfs<F: FnMut() -> ()>(level: u8, prev_move: Option<Turn>, prev_cube: Cube, f: &mut F) {
    let iter = match prev_move {
        Some(x) => { x.into_iter() },
        None => { Turn::allturns() },
    };
    for t in iter {
        let cube = prev_cube * t;
        if level != 0 {
            dfs(level - 1, Some(t), cube, f);
        } else {
            f();
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
    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{:?}: {}", t, cube * t);
        println!("{:?} cycles: {}", t, (cube * t).cycles());
    }
    println!("sledge: {}", cube.turns(&[R3, F1, R1, F3]));
    println!("sledge cycles: {}", cube.turns(&[R3, F1, R1, F3]).cycles());

    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{:?} corner parity={}", t, (corners * t).parity());
    }
    let s = Cube::default().turns(&[R1, U1]);
    println!("R U: {}", s.cycles());
    println!("F R U R' U' F': {}", cube.turns(&[F1, R1, U1, R3, U3, F3]).cycles());
    println!("F R U R' U' F' U: {}", cube.turns(&[F1, R1, U1, R3, U3, F3, U1]).cycles());

    println!("{}", "R U R' U'".parse::<Turns>().unwrap());
    let t = "F    R U R' U' F'".parse::<Turns>().unwrap();
    println!("{t}: {}", cube * &t);
    println!("{t}: {}", (cube * &t).cycles());

    for x in [U1, U2, R1, F1, D1, L1, B1] {
        let v: Vec<_> = x.into_iter().collect();
        println!("{:?}: {:?}", x, v);
    }
    println!("allturns: {:?}", Turn::allturns().collect::<Vec<_>>());
    for x in Turn::allturns() {
        println!("{}: {}", x, (cube * x).cycles());
    }

    println!("bare DFS...");
    for i in 0..7 {
        let now = Instant::now();
        let mut count = 0u64;
        dfs(i, None, cube, &mut || { count = count + 1; });
        println!("level {} count {} {:.2?}", i+1, count, now.elapsed());
    }

    let now = Instant::now();
    let hash = RefCell::new(BTreeMap::<u128, u8>::new());
    let mut v = vec![0usize; 7];
    {
        let mut muthash = hash.borrow_mut();
        muthash.insert(cube.pack(), 0);
    }
    println!("counting unique positions...");
    do_level(1, 6, U1, cube, &hash);
    println!("reducing...");
    let muthash = hash.borrow_mut();
    for e in muthash.values() {
        v[*e as usize] += 1;
    }
    println!("{:?}", v);
    println!("{:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use Turn::*;

    #[test]
    fn test_inverses() {
        let corners = corners_neon::Corners::default();
        let edges = edges_neon::Edges::default();
        let cube = Cube::default();
        for (lhs, rhs) in [
            (U1, U3), (U2, U2), (U3, U1),
            (R1, R3), (R2, R2), (R3, R1),
            (F1, F3), (F2, F2), (F3, F1),
            (D1, D3), (D2, D2), (D3, D1),
            (B1, B3), (B2, B2), (B3, B1),
            (L1, L3), (L2, L2), (L3, L1),
        ] {
            assert_eq!(corners * lhs, !(corners * rhs));
            assert_eq!(edges * lhs, !(edges * rhs));
            assert_eq!(cube * lhs, !(cube * rhs));
        }
    }

    #[test]
    fn test_parity() {
        let corners = corners_neon::Corners::default();
        let edges = edges_neon::Edges::default();
        for (lhs, rhs) in [
            (U1, true), (U2, false), (U3, true),
            (R1, true), (R2, false), (R3, true),
            (F1, true), (F2, false), (F3, true),
            (D1, true), (D2, false), (D3, true),
            (B1, true), (B2, false), (B3, true),
            (L1, true), (L2, false), (L3, true),
            ] {
                assert_eq!((corners * lhs).parity(), rhs);
                assert_eq!((edges * lhs).parity(), rhs);
        }
        assert!(!(corners * &Turns::from(&[R3, F1, R1, F3])).parity());
    }

    #[test]
    fn test_mul() {
        let corners = corners_neon::Corners::default();
        let s = corners * &Turns::from(&[R1, U1, R3, U3]);
        assert_eq!(s * s, corners * &Turns::from(&[R1, U1, R3, U3, R1, U1, R3, U3]));
    }
}
