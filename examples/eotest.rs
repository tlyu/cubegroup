use cubegroup::*;

fn main() {
    for t in allturns() {
        println!("{t}");
        for i in 0u16..2048 {
            let c = edges_neon::Edges::set_eo(i) * t;
            println!("{i:011b} {:011b} {}", c.eo(), c.cycles().speffz());
            let ca = edges_array::Edges::set_eo(i) * t;
            assert_eq!(c.eo(), ca.eo());
        }
    }
}
