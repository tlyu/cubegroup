use cubegroup::*;
use cubegroup::dr::*;

fn main() {
    for t in allturns() {
        println!("{t}");
        for i in 0u16..NEO {
            let c = edges_neon::Edges::from_eo(i) * t;
            println!("{i:011b} {:011b} {}", c.eo(), c.cycles().speffz());
            let ca = edges_array::Edges::from_eo(i) * t;
            assert_eq!(c.eo(), ca.eo());
        }
    }
}
