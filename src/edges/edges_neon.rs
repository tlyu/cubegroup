#![cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::*;
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::Hasher;
use std::ops::{Mul, Not};

use bytemuck::*;

use super::*;
use crate::simd_util::*;

const EP_MASK: uint8x16_t = unsafe { Load8x16 { b: [0x0f; 16] } .a };
const EO_MASK: uint8x16_t = unsafe { Load8x16 { b: [0x10; 16] } .a };
const EDGES_IDENT: uint8x16_t = unsafe { Load8x16 { qq: 0x0f0e0d0c0b0a09080706050403020100 } .a };

macro_rules! edges {
    ( $( ($id:expr, $flip:expr) ),* ) => {
        Edges(unsafe { Load8x16 { b: [
            $( $id | ($flip << 4) ),*
            , 12, 13, 14, 15
        ]} .a })
    }
}

static EDGE_TURNS: [Edges; NTURNS] = edge_turns!();

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(transparent)]
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
        let x = unsafe { Load8x16 { a: self.0 } .qq };
        x.hash(state)
    }
}
impl Eq for Edges {}
impl PartialEq for Edges {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { Load8x16 { a: self.0 } .qq == Load8x16 { a: rhs.0 } .qq}
    }
}
impl Ord for Edges {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = unsafe { Load8x16 { a: self.0 } .qq };
        let b = unsafe { Load8x16 { a: other.0 } .qq };
        a.cmp(&b)
    }
}
impl PartialOrd for Edges {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Mul for Edges {
    type Output = Edges;
    #[inline]
    fn mul(self, rhs: Edges) -> Edges {
        let mut out = unsafe { vqtbl1q_u8(self.0, vandq_u8(rhs.0, EP_MASK)) };
        out = unsafe { veorq_u8(out, vandq_u8(rhs.0, EO_MASK)) };
        Edges(out)
    }
}
impl Mul<Turn> for Edges {
    type Output = Edges;
    #[inline]
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
        let mut out = Load8x16 { a: EDGES_IDENT };
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
        let a: &[u8; 16] = must_cast_ref(&x);
        let mut out = [0u8; NEDGES];
        out.copy_from_slice(&a[..NEDGES]);
        must_cast(out)
    }
}
impl From<edges_array::Edges> for Edges {
    fn from(v: edges_array::Edges) -> Self {
        let a: &[u8; NEDGES] = must_cast_ref(&v);
        let mut out = [0u8; 16];
        out[..NEDGES].copy_from_slice(a);
        // Copy upper bytes from EDGES_IDENT
        must_cast(unsafe { vorrq_u8(must_cast(out), EDGES_IDENT) })
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
        out |= (a >> 3) & (0x1f << 5);
        out |= (a >> 6) & (0x1f << 10);
        out |= (a >> 9) & (0x1f << 15);
        out |= (a >> 12) & (0x1f << 20);
        out |= (a >> 15) & (0x1f << 25);
        out |= (a >> 18) & (0x1f << 30);
        out |= (a >> 21) & (0x1f << 35);
        out |= (a >> 24) & (0x1f << 40);
        out |= (a >> 27) & (0x1f << 45);
        out |= (a >> 30) & (0x1f << 50);
        out |= (a >> 33) & (0x1f << 55);
        out as u64
    }
    fn speffz(self) -> String {
        edges_array::Edges::from(self).speffz()
    }
    fn net_flip(&self) -> u8 {
        unsafe { Load8x16 { a: self.0 } .qq }.count_ones() as u8 & 1
    }
    fn from_speffz(s: &str) -> Result<Self, ()> {
        Ok(edges_array::Edges::from_speffz(s)?.into())
    }
}
