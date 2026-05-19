#![cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Mul, Not};

use bytemuck::*;

use super::*;
use crate::*;

const CP_MASK: uint8x8_t = must_cast([0x07u8; 8]);
const CO_MASK: uint8x8_t = must_cast([0x18u8; 8]);
const CORNERS_IDENT: uint8x8_t = must_cast(0x0706050403020100u64);

macro_rules! corners {
    ($(($id:expr, $twist:expr)),*) => {
        Corners(must_cast([
                $( $id as u8 | (($twist as u8) << 3) ),*
            ]))
    }
}

static CORNER_TURNS: [Corners; NTURNS] = corner_turns!();

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(transparent)]
pub struct Corners(uint8x8_t);

impl Default for Corners {
    fn default() -> Corners {
        Corners(CORNERS_IDENT)
    }
}
impl From<Corners> for corners_array::Corners {
    fn from(x: Corners) -> Self {
        must_cast(x)
    }
}
impl Hash for Corners {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        let x: u64 = must_cast(*self);
        x.hash(state)
    }
}
impl Eq for Corners {}
impl PartialEq for Corners {
    fn eq(&self, rhs: &Self) -> bool {
        let a: u64 = must_cast(*self);
        a == must_cast(*rhs)
    }
}
impl Ord for Corners {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: u64 = must_cast(*self);
        let b: u64 = must_cast(*other);
        a.cmp(&b)
    }
}
impl PartialOrd for Corners {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Not for Corners {
    type Output = Corners;
    fn not(self) -> Self {
        let mut out = [0u8; 8];
        let a: [u8; NCORNERS] = must_cast(self);
        for i in 0..NCORNERS {
            let slot = a[i] as usize & 0x07;
            let mut twist = a[i] & 0x18;
            // Negate twist mod 3
            if twist != 0x00 {
                twist ^= 0x18;
            }
            out[slot] = i as u8 | twist;
        }
        must_cast(out)
    }
}
// Use target_feature to avoid annotating everything as unsafe
#[target_feature(enable = "neon")]
#[inline]
fn unsafe_mul(a: Corners, b: Corners) -> Corners {
    let mut out = vtbl1_u8(a.0, vand_u8(b.0, CP_MASK));
    out = vadd_u8(out, vand_u8(b.0, CO_MASK));
    // Carry adjustment for twists from Andrew Skalski's vcube,
    // by way of ArhanChaudhary
    out = vmin_u8(out, vsub_u8(out, CO_MASK));
    Corners(out)
}
impl Mul for Corners {
    type Output = Corners;
    #[inline]
    fn mul(self, rhs: Self) -> Corners {
        unsafe { unsafe_mul(self, rhs) }
    }
}
impl Mul<Turn> for Corners {
    type Output = Corners;
    #[inline]
    fn mul(self, rhs: Turn) -> Corners {
        self * CORNER_TURNS[rhs]
    }
}
impl From<corners_array::Corners> for Corners {
    fn from(x: corners_array::Corners) -> Corners {
        must_cast(x)
    }
}
gen_ops! {
    Corners
}
impl CubeOps for Corners {}
impl CornersTrait for Corners {
    type Cycles = corners_array::CornerCycles;
    fn parity(&self) -> bool {
        corners_array::Corners::from(*self).parity()
    }
    fn cycles(&self) -> corners_array::CornerCycles {
        corners_array::Corners::from(*self).cycles()
    }
    fn pack(&self) -> u64 {
        let a: u64 = must_cast(self.0);
        let mut out = a & 0x1f;
        out |= (a >> 3) & (0x1f << 5);
        out |= (a >> 6) & (0x1f << 10);
        out |= (a >> 9) & (0x1f << 15);
        out |= (a >> 12) & (0x1f << 20);
        out |= (a >> 15) & (0x1f << 25);
        out |= (a >> 18) & (0x1f << 30);
        out |= (a >> 21) & (0x1f << 35);
        out
    }
    fn net_twist(&self) -> u8 {
        unsafe { ((vaddv_u8(vand_u8(self.0, CO_MASK))) >> 3) % 3 }
    }
}
