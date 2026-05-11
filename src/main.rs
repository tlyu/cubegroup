use cube_foo::*;
fn main() {
    use Turn::*;
    let corners = Corners::default();
    println!("{corners}");
    println!("U1: {}", corners * U1);
    println!("U2: {}", corners * U2);
    println!("U3: {}", corners * U3);
    println!("R1: {}", corners * R1);
    println!("R2: {}", corners * R2);
    println!("R3: {}", corners * R3);

    println!("R1U1R3U3R1U1R3U3: {}", corners.turns(&[R1, U1, R3, U3, R1, U1, R3, U3]));
    println!("sledge: {}", corners.turns(&[R3, F1, R1, F3]));

    let edges = Edges::default();
    println!("{edges}");
    println!("U1: {}", edges * U1);
    println!("U2: {}", edges * U2);
    println!("U3: {}", edges * U3);
    println!("R1: {}", edges * R1);
    println!("R2: {}", edges * R2);
    println!("R3: {}", edges * R3);

    println!("sledge: {}", edges.turns(&[R3, F1, R1, F3]));

    assert_eq!(corners * U2, !(corners * U2));
    assert_eq!(corners * U3, !(corners * U1));
    assert_eq!(corners * R2, !(corners * R2));
    assert_eq!(corners * R3, !(corners * R1));
    assert_eq!(corners * F3, !(corners * F1));

    assert_eq!(edges * U3, !(edges * U1));
    assert_eq!(edges * U2, !(edges * U2));
    assert_eq!(edges * R3, !(edges * R1));
    assert_eq!(edges * R2, !(edges * R2));
    assert_eq!(edges * F3, !(edges * F1));
    assert_eq!(edges * F2, !(edges * F2));
    assert_eq!(edges * D3, !(edges * D1));
    assert_eq!(edges * D2, !(edges * D2));
    assert_eq!(edges * B3, !(edges * B1));
    assert_eq!(edges * B2, !(edges * B2));
    assert_eq!(edges * L3, !(edges * L1));
    assert_eq!(edges * L2, !(edges * L2));

    let s = corners.turns(&[R1, U1, R3, U3]);
    assert_eq!(s * s, corners.turns(&[R1, U1, R3, U3, R1, U1, R3, U3]));
}
