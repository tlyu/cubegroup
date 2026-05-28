use cubegroup::*;
use cubegroup::dr::*;

// Expand base 3 digits as base 16 for better display
fn semi(co: u16) -> u32 {
    let mut out = 0u32;
    for i in 0u32..7 {
        out |= (((co / 3u16.pow(i)) % 3) as u32) << (4*i);
    }
    out
}

fn main() {
    for t in allturns() {
        println!("{t}");
        for i in 0u16..NCO {
            let c = corners_neon::Corners::from_co(i) * t;
            println!("{:07x} {:07x} {}", semi(i), semi(c.co()), c.cycles().speffz());
            let ca = corners_array::Corners::from_co(i) * t;
            assert_eq!(c.co(), ca.co());
        }
    }
}
