use cubegroup::*;

fn main() {
    for t in allturns() {
        println!("{t}");
        for i in 0u16..2048 {
            let parity = ((i.count_ones() & 1) << 11) as u16;
            let eo = parity | i;
            let c = edges_neon::Edges::set_eo(eo) * t;
            println!("{eo:012b} {:012b} {}", c.eo(), c.cycles().speffz());
            let ca = edges_array::Edges::set_eo(eo) * t;
            assert_eq!(c.eo(), ca.eo());
        }
    }
}
