#![cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::*;
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::Hasher;
use std::ops::{Mul, Not};

use bytemuck::*;

use super::*;

const EP_MASK: uint8x16_t = must_cast([0x0fu8; 16]);
const EO_MASK: uint8x16_t = must_cast([0x10u8; 16]);
// The extra numbers allow the permutations to keep the upper bytes constant
const EDGES_IDENT: uint8x16_t = must_cast([0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

macro_rules! edges {
    ( $( ($id:expr, $flip:expr) ),* ) => {
        Edges(must_cast([
            $( $id as u8 | (($flip << 4) as u8) ),*
            , 12, 13, 14, 15
        ]))
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
        let x: &u128 = must_cast_ref(self);
        x.hash(state)
    }
}
impl Eq for Edges {}
impl PartialEq for Edges {
    fn eq(&self, rhs: &Self) -> bool {
        let a: &u128 = must_cast_ref(self);
        a == must_cast_ref(rhs)
    }
}
impl Ord for Edges {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: &u128 = must_cast_ref(self);
        a.cmp(must_cast_ref(other))
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
        let mut out: [u8; 16] = must_cast(EDGES_IDENT);
        let a: [u8; 16] = must_cast(self);
        for i in 0..NEDGES {
            let v = a[i];
            out[v as usize & 0xf] = i as u8 | v & 0x10;
        }
        must_cast(out)
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
        let mut out: [u8; 16] = must_cast(EDGES_IDENT);
        out[..NEDGES].copy_from_slice(a);
        must_cast(out)
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
        let a: u128 = must_cast(*self);
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
        let x: u128 = must_cast(unsafe { vandq_u8(self.0, EO_MASK) });
        x.count_ones() as u8 & 1
    }
    fn from_speffz(s: &str) -> Result<Self, ()> {
        Ok(edges_array::Edges::from_speffz(s)?.into())
    }
}
