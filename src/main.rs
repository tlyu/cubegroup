use std::cell::RefCell;
use std::collections::HashSet;
use std::str::FromStr;

use cube_foo::*;

fn do_level(level: usize, max_level: usize, prev_move: Turn, prev_cube: Cube, counts: &Vec<RefCell<u64>>, hash: &RefCell<HashSet<Cube>>) {
    let mut count = counts[level].borrow_mut();
    let mut iter = if level < 2 { Turn::allturns() } else { prev_move.into_iter() };
    let mut v = Vec::<(Turn, Cube)>::new();
    {
        let mut muthash = hash.borrow_mut();
        for t in iter {
            let cube = prev_cube * t;
            v.push((t, cube));
            if muthash.insert(cube) {
                *count += 1;
            }
        }
    }
    println!("level {:width$} move {} count {}", level, prev_move, count, width = (level as usize));
    if level == max_level {
        return;
    }
    // iter = if level < 2 { Turn::allturns() } else { prev_move.into_iter() };
    for (t, cube) in v {
        do_level(level + 1, max_level, t, cube, counts, hash);
    }
}

fn main() {
    use Turn::*;
    let corners = Corners::default();
    println!("default corners: {corners}");
    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{}: {}", t, corners * t);
        println!("{} cycles: {}", t, (corners * t).cycles());
    }
    let s = "R";
    println!("{s}: {}", (corners * Turn::from_str(s).unwrap()).cycles());
    println!("sledge: {}", corners.turns(&[R3, F1, R1, F3]));
    println!("sledge cycles: {}", corners.turns(&[R3, F1, R1, F3]).cycles());
    println!("double sledge cycles: {}", corners.turns(&[R3, F1, R1, F3, R3, F1, R1, F3]).cycles());

    let edges = Edges::default();
    println!("default edges: {edges}");
    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{:?}: {}", t, edges * t);
        println!("{:?} cycles: {}", t, (edges * t).cycles());
    }
    println!("sledge: {}", edges.turns(&[R3, F1, R1, F3]));
    println!("sledge cycles: {}", edges.turns(&[R3, F1, R1, F3]).cycles());

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

    let counts = vec![RefCell::new(0u64); 7];
    let hash = RefCell::new(HashSet::<Cube>::new());
    do_level(1, 4, U1, Cube::default(), &counts, &hash);
}

#[cfg(test)]
mod tests {
    use super::*;
    use Turn::*;

    #[test]
    fn test_inverses() {
        let corners = Corners::default();
        let edges = Edges::default();
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
        let corners = Corners::default();
        let edges = Edges::default();
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
        assert!(!(corners.turns(&[R3, F1, R1, F3])).parity());
    }

    #[test]
    fn test_mul() {
        let corners = Corners::default();
        let s = corners.turns(&[R1, U1, R3, U3]);
        assert_eq!(s * s, corners.turns(&[R1, U1, R3, U3, R1, U1, R3, U3]));
    }
}
