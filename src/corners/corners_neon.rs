#![cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::*;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Mul, Not};

use super::*;
use crate::simd_util::*;
use crate::{Turn, Turns};

const CP_MASK: uint8x8_t = unsafe { Load8x8 { q: 0x0707070707070707 } .a };
const CO_MASK: uint8x8_t = unsafe { Load8x8 { q: 0x1818181818181818 } .a };
const CORNERS_IDENT: uint8x8_t = unsafe { Load8x8 { q: 0x0706050403020100 } .a };
// SIMD table lookups will trash the upper bits; clear them for compares
const CMP_MASK: u64 = 0x1f1f1f1f1f1f1f1f;

macro_rules! corners {
    ($(($id:expr, $twist:expr)),*) => {
        Corners(unsafe {
            Load8x8 { b: [
                $( $id | ($twist << 3) ),*
            ] } .a
        })
    }
}
static CORNER_TURNS: [Corners; 18] = corner_turns!();

#[derive(Clone, Copy, Debug)]
pub struct Corners(uint8x8_t);
impl Default for Corners {
    fn default() -> Corners {
        Corners(CORNERS_IDENT)
    }
}
impl Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", corners_array::Corners::from(*self))
    }
}
impl From<Corners> for corners_array::Corners {
    fn from(x: Corners) -> Self {
        let a = unsafe { Load8x8 { a: x.0 } .b };
        let v: Vec<_> = a.into_iter().map(corners_array::Corner).collect();
        let out: [_; NCORNERS] = v[..NCORNERS].try_into().unwrap();
        corners_array::Corners(out)
    }
}
impl Hash for Corners {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        let x = unsafe { Load8x8 { a: self.0 } .q & CMP_MASK };
        x.hash(state)
    }
}
impl Eq for Corners {}
impl PartialEq for Corners {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { Load8x8 { a: self.0 } .q & CMP_MASK == Load8x8 { a: rhs.0 } .q & CMP_MASK }
    }
}
impl Not for Corners {
    type Output = Corners;
    fn not(self) -> Self {
        let mut out = Load8x8 { q: 0u64 };
        let a = unsafe { Load8x8 { a: self.0 } .b };
        for i in 0..NCORNERS {
            let slot = a[i] as usize & 0x07;
            let mut twist = a[i] & 0x18;
            // Negate twist mod 3
            if twist != 0x00 {
                twist ^= 0x18;
            }
            unsafe { out.b[slot] = i as u8 | twist };
        }
        Corners(unsafe { out.a })
    }
}
// Use target_feature to avoid annotating everything as unsafe
#[target_feature(enable = "neon")]
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
    fn mul(self, rhs: Self) -> Corners {
        unsafe { unsafe_mul(self, rhs) }
    }
}
impl Mul<Turn> for Corners {
    type Output = Corners;
    fn mul(self, rhs: Turn) -> Corners {
        self * CORNER_TURNS[rhs]
    }
}
impl Mul<&Turns> for Corners {
    type Output = Corners;
    fn mul(self, rhs: &Turns) -> Corners {
        let mut out = self;
        for x in &rhs.0 {
            out = out * *x;
        }
        out
    }
}
impl CornersTrait for Corners {
    type Cycles = corners_array::CornerCycles;
    fn parity(&self) -> bool {
        corners_array::Corners::from(*self).parity()
    }
    fn cycles(&self) -> corners_array::CornerCycles {
        corners_array::Corners::from(*self).cycles()
    }
    fn pack(&self) -> u64 {
        let a = unsafe { Load8x8 { a: self.0 } .q };
        let mut out = a & 0x1f;
        out |= (a >> 8) & (0x1f << 5);
        out |= (a >> 16) & (0x1f << 10);
        out |= (a >> 24) & (0x1f << 15);
        out |= (a >> 32) & (0x1f << 20);
        out |= (a >> 40) & (0x1f << 25);
        out |= (a >> 48) & (0x1f << 30);
        out |= (a >> 56) & (0x1f << 35);
        out
    }
}
