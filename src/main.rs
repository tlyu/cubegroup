use cube_foo::*;
fn main() {
    use Turn::*;
    let corners = Corners::default();
    println!("default corners: {corners}");
    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{:?}: {}", t, corners * t);
    }
    println!("sledge: {}", corners.turns(&[R3, F1, R1, F3]));

    let edges = Edges::default();
    println!("default edges: {edges}");
    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{:?}: {}", t, edges * t);
    }
    println!("sledge: {}", edges.turns(&[R3, F1, R1, F3]));

    let cube = Cube::default();
    println!("default cube: {cube}");
    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{:?}: {}", t, cube * t);
    }
    println!("sledge: {}", cube.turns(&[R3, F1, R1, F3]));

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

    let s = corners.turns(&[R1, U1, R3, U3]);
    assert_eq!(s * s, corners.turns(&[R1, U1, R3, U3, R1, U1, R3, U3]));

    for t in [U1, U2, U3, R1, R2, R3, F1, F2, F3] {
        println!("{:?} parity={}", t, (corners * t).parity());
    }
    for (lhs, rhs) in [
        (U1, true), (U2, false), (U3, true),
        (R1, true), (R2, false), (R3, true),
        (F1, true), (F2, false), (F3, true),
        (D1, true), (D2, false), (D3, true),
        (B1, true), (B2, false), (B3, true),
        (L1, true), (L2, false), (L3, true),
    ] {
        assert_eq!((corners * lhs).parity(), rhs);
    }
    assert!(!(corners.turns(&[R3, F1, R1, F3])).parity());
}
