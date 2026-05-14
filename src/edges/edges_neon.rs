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
impl Mul<Turn> for Edges {
    type Output = Edges;
    fn mul(self, rhs: Turn) -> Edges {
        self * EDGE_TURNS[rhs]
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
impl EdgesTrait for Edges {
    type Cycles = edges_array::EdgeCycles;
    fn parity(&self) -> bool {
        edges_array::Edges::from(*self).parity()
    }
    fn cycles(&self) -> edges_array::EdgeCycles {
        edges_array::Edges::from(*self).cycles()
    }
    fn pack(&self) -> u64 {
        let a = unsafe { Load8x16 { a: self.0 } .qq };
        let mut out = a & 0x1f;
        out |= (a >> 8) & (0x1f << 5);
        out |= (a >> 16) & (0x1f << 10);
        out |= (a >> 24) & (0x1f << 15);
        out |= (a >> 32) & (0x1f << 20);
        out |= (a >> 40) & (0x1f << 25);
        out |= (a >> 48) & (0x1f << 30);
        out |= (a >> 56) & (0x1f << 35);
        out |= (a >> 64) & (0x1f << 40);
        out |= (a >> 72) & (0x1f << 45);
        out |= (a >> 80) & (0x1f << 50);
        out |= (a >> 96) & (0x1f << 55);
        out as u64
    }
}
