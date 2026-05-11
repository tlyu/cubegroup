use cube_foo::*;
fn main() {
    use Turn::*;
    let corners = Corners::default();
    println!("{corners}");
    println!("U1: {}", corners.turn(U1));
    println!("U2: {}", corners.turn(U2));
    println!("U3: {}", corners.turn(U3));
    println!("R1: {}", corners.turn(R1));
    println!("R2: {}", corners.turn(R2));
    println!("R3: {}", corners.turn(R3));

    println!("R1U1R3U3R1U1R3U3: {}", corners.turns(&[R1, U1, R3, U3, R1, U1, R3, U3]));
    println!("sledge: {}", corners.turns(&[R3, F1, R1, F3]));

    let edges = Edges::default();
    println!("{edges}");
    println!("U1: {}", edges.turn(U1));
    println!("U2: {}", edges.turn(U2));
    println!("U3: {}", edges.turn(U3));
    println!("R1: {}", edges.turn(R1));
    println!("R2: {}", edges.turn(R2));
    println!("R3: {}", edges.turn(R3));

    println!("sledge: {}", edges.turns(&[R3, F1, R1, F3]));

    assert_eq!(corners.turn(U2), !corners.turn(U2));
    assert_eq!(corners.turn(U3), !corners.turn(U1));
    assert_eq!(corners.turn(R2), !corners.turn(R2));
    assert_eq!(corners.turn(R3), !corners.turn(R1));
    assert_eq!(corners.turn(F3), !corners.turn(F1));

    assert_eq!(edges.turn(U3), !edges.turn(U1));
    assert_eq!(edges.turn(U2), !edges.turn(U2));
    assert_eq!(edges.turn(R3), !edges.turn(R1));
    assert_eq!(edges.turn(R2), !edges.turn(R2));
    assert_eq!(edges.turn(F3), !edges.turn(F1));
    assert_eq!(edges.turn(F2), !edges.turn(F2));
    assert_eq!(edges.turn(D3), !edges.turn(D1));
    assert_eq!(edges.turn(D2), !edges.turn(D2));
    assert_eq!(edges.turn(B3), !edges.turn(B1));
    assert_eq!(edges.turn(B2), !edges.turn(B2));
    assert_eq!(edges.turn(L3), !edges.turn(L1));
    assert_eq!(edges.turn(L2), !edges.turn(L2));

    let s = corners.turns(&[R1, U1, R3, U3]);
    assert_eq!(s * s, corners.turns(&[R1, U1, R3, U3, R1, U1, R3, U3]));
}
