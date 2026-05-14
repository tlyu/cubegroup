#![cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::*;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Mul, Not};

use super::*;
use crate::simd_util::*;
use crate::{Turn, Turns};

const CP_MASK: uint8x8_t = unsafe { Load8x8 { q: 0x0707070707070707 } .a };
const CO_MASK: uint8x8_t = unsafe { Load8x8 { q: 0x3030303030303030 } .a };
const CO_ADJ: uint8x8_t = unsafe { Load8x8 { q: 0x1010101010101010 } .a };
const CO_OVF_MASK: uint8x8_t = unsafe { Load8x8 { q: 0x4040404040404040 } .a};
const CORNERS_IDENT: uint8x8_t = unsafe { Load8x8 { q: 0x0706050403020100 } .a };
// SIMD table lookups will trash the upper bits; clear them for compares
const CMP_MASK: u64 = 0x3737373737373737;

macro_rules! corners {
    ($(($id:expr, $twist:expr)),*) => {
        Corners(unsafe {
            Load8x8 { b: [
                $( $id | ($twist << 4) ),*
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
            let mut twist = a[i] & 0x30;
            // Negate twist mod 3
            if twist != 0x00 {
                twist ^= 0x30;
            }
            unsafe { out.b[slot] = i as u8 | twist };
        }
        Corners(unsafe { out.a })
    }
}
impl Mul for Corners {
    type Output = Corners;
    fn mul(self, rhs: Self) -> Corners {
        let mut out = unsafe { vtbl1_u8(self.0, vand_u8(rhs.0, CP_MASK))};
 
        // Trick from Joba's cubelib to avoid remainder ops.
        // This requires a spare bit for the orientation field.

        // Add 1 to each corner orientation, so modulo 3 wrap will give 0b1xx
        let co = unsafe { vadd_u8(vand_u8(rhs.0, CO_MASK), CO_ADJ) };
        out = unsafe { vadd_u8(out, co) };
        // All the overflow bits, giving 4 for each overflow
        let ovf = unsafe { vand_u8(out, CO_OVF_MASK) };
        // Negate overflow bits and shift right, giving 1 for each non-overflow
        let novf = unsafe { vshr_n_u8::<2>(vand_u8(vmvn_u8(ovf), CO_OVF_MASK)) };

        // Subtract the orientation adjustments.
        // This undoes the original +1 adjustment, and corrects overflows.
        out = unsafe { vsub_u8(out, vorr_u8(ovf, novf)) };
        Corners(out)
    }
}
impl Mul<&Corners> for &Corners {
    type Output = Corners;
    fn mul(self, rhs: &Corners) -> Corners {
        *self * *rhs
    }
}
impl Mul<Turn> for Corners {
    type Output = Corners;
    fn mul(self, rhs: Turn) -> Corners {
        self * CORNER_TURNS[rhs]
    }
}
impl Mul<Turn> for &Corners {
    type Output = Corners;
    fn mul(self, rhs: Turn) -> Corners {
        *self * rhs
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
impl Mul<&Turns> for &Corners {
    type Output = Corners;
    fn mul(self, rhs: &Turns) -> Corners {
        *self * rhs
    }
}
impl CornersTrait<CornerCycles> for Corners {
    fn parity(&self) -> bool {
        corners_array::Corners::from(*self).parity()
    }
    fn cycles(&self) -> CornerCycles {
        corners_array::Corners::from(*self).cycles()
    }
}
