#![cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::*;
use std::fmt::{self, Display};
use std::hash::Hasher;
use std::ops::{Mul, Not};

use super::*;
use crate::simd_util::*;
use crate::{Turn, Turns};

const EP_MASK: uint8x16_t = unsafe { Load8x16 { qq: 0x0f0f0f0f0f0f0f0f0f0f0f0f } .a };
const EO_MASK: uint8x16_t = unsafe { Load8x16 { qq: 0x101010101010101010101010 } .a };
const EDGES_IDENT: uint8x16_t = unsafe { Load8x16 { qq: 0x0b0a09080706050403020100 } .a };
// SIMD table lookups will trash the upper bits; clear them for compares
const CMP_MASK: u128 = 0x1f1f1f1f1f1f1f1f1f1f1f1f;

macro_rules! edges {
    ( $( ($id:expr, $flip:expr) ),* ) => {
        Edges(unsafe { Load8x16 { b: [
            $( $id | ($flip << 4) ),*
            , 0, 0, 0, 0
        ]} .a })
    }
}

static EDGE_TURNS: [Edges; 18] = edge_turns!();

#[derive(Clone, Copy, Debug)]
pub struct Edges(uint8x16_t);
impl Default for Edges {
    fn default() -> Self {
        Edges(EDGES_IDENT)
    }
}
impl Display for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", edges_array::Edges::from(*self))
    }
}
impl Hash for Edges {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        let x = unsafe { Load8x16 { a: self.0 } .qq & CMP_MASK };
        x.hash(state)
    }
}
impl Eq for Edges {}
impl PartialEq for Edges {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { Load8x16 { a: self.0 } .qq & CMP_MASK == Load8x16 { a: rhs.0 } .qq & CMP_MASK }
    }
}
impl Mul for Edges {
    type Output = Edges;
    fn mul(self, rhs: Edges) -> Edges {
        let mut out = unsafe { vqtbl1q_u8(self.0, vandq_u8(rhs.0, EP_MASK)) };
        out = unsafe { veorq_u8(out, vandq_u8(rhs.0, EO_MASK)) };
        Edges(out)
    }
}
impl Mul<&Edges> for &Edges {
    type Output = Edges;
    fn mul(self, rhs: &Edges) -> Edges {
        *self * *rhs
    }
}
impl Mul<Turn> for Edges {
    type Output = Edges;
    fn mul(self, rhs: Turn) -> Edges {
        self * EDGE_TURNS[rhs]
    }
}
impl Mul<Turn> for &Edges {
    type Output = Edges;
    fn mul(self, rhs: Turn) -> Edges {
        *self * rhs
    }
}
impl Mul<&Turns> for Edges {
    type Output = Edges;
    fn mul(self, rhs: &Turns) -> Edges {
        let mut out = self;
        for x in &rhs.0 {
            out = out * *x;
        }
        out
    }
}
impl Mul<&Turns> for &Edges {
    type Output = Edges;
    fn mul(self, rhs: &Turns) -> Edges {
        *self * rhs
    }
}
impl Not for Edges {
    type Output = Edges;
    fn not(self) -> Edges {
        let mut out = Load8x16 { qq: 0u128 };
        let a = unsafe { Load8x16 { a: self.0 } .b };
        for i in 0..NEDGES {
            let v = a[i];
            unsafe { out.b[v as usize & 0xf] = i as u8 | v & 0x10 };
        }
        Edges(unsafe { out.a })
    }
}
impl From<Edges> for edges_array::Edges {
    fn from(x: Edges) -> edges_array::Edges {
        let a = unsafe { Load8x16 { a: x.0 } .b };
        let v: Vec<_> = a.into_iter().map(edges_array::Edge).collect();
        let out: [_; NEDGES] = v[..NEDGES].try_into().unwrap();
        edges_array::Edges(out)
    }
}
impl EdgesTrait<edges_array::EdgeCycles> for Edges {
    fn parity(&self) -> bool {
        edges_array::Edges::from(*self).parity()
    }
    fn cycles(&self) -> edges_array::EdgeCycles {
        edges_array::Edges::from(*self).cycles()
    }
    fn pack(&self) -> u64 {
        let a = unsafe { Load8x16 { a: self.0 } .qq };
        let mut out = a & ((1 << 5) - 1);
        out |= (a >> 5) & ((1 << 10) - 1);
        out |= (a >> 10) & ((1 << 15) - 1);
        out |= (a >> 15) & ((1 << 20) - 1);
        out |= (a >> 20) & ((1 << 25) - 1);
        out |= (a >> 25) & ((1 << 30) - 1);
        out |= (a >> 30) & ((1 << 35) - 1);
        out |= (a >> 35) & ((1 << 40) - 1);
        out |= (a >> 40) & ((1 << 45) - 1);
        out |= (a >> 45) & ((1 << 50) - 1);
        out |= (a >> 50) & ((1 << 55) - 1);
        out |= (a >> 55) & ((1 << 60) - 1);
        out as u64
    }
}
