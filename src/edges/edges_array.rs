use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

use bytemuck::*;
use itertools::Itertools;

use crate::*;
use crate::speffz::*;
use crate::Turn;
use crate::CubeOps;

use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Pod, Zeroable)]
#[repr(transparent)]
pub struct Edges(pub(crate) [Edge; 12]);
const EDGES_IDENT: Edges = must_cast([0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

impl Default for Edges {
    fn default() -> Self {
        EDGES_IDENT
    }
}
impl Index<u8> for Edges {
    type Output = Edge;
    fn index(&self, i: u8) -> &Edge {
        &self.0[i as usize]
    }
}
impl IndexMut<u8> for Edges {
    fn index_mut(&mut self, i: u8) -> &mut Edge {
        &mut self.0[i as usize]
    }
}
impl Mul for Edges {
    type Output = Edges;
    fn mul(self, rhs: Self) -> Edges {
        Edges(rhs.0.map(|x| Edge(self[x.0 & 0xf].0 ^ (x.0 & 0x10))))
    }
}
impl Mul<Turn> for Edges {
    type Output = Edges;
    fn mul(self, rhs: Turn) -> Edges {
        self * EDGE_TURNS[rhs]
    }
}
impl Not for Edges {
    type Output = Edges;
    fn not(self) -> Self {
        let mut out = Edges([Edge(0); 12]);
        for i in 0..12 {
            let e = self[i].0;
            out[e & 0xf] = Edge(i | (e & 0x10));
        }
        out
    }
}
gen_ops! {
    Edges
}
impl CubeOps for Edges {}
impl EdgesTrait for Edges {
    type Cycles = EdgeCycles;
    fn parity(&self) -> bool {
        let mut unseen = (1u16 << NEDGES) - 1;
        let mut i = 0u8;
        let mut out = false;
        while unseen != 0 {
            unseen &= !(1 << i);
            // Follow a cycle
            if self[i].0 & 0xf != i {
                i = self[i].0 & 0xf;
                // Only toggle parity if not coming back to the cycle start
                if (unseen & (1 << i)) != 0 {
                    out = !out;
                    continue;
                }
            }
            // Otherwise, find the lowest unseen piece
            i = unseen.trailing_zeros() as u8;
        }
        out
    }
    fn cycles(&self) -> EdgeCycles {
        let mut unseen = (1u16 << NEDGES) - 1;
        let mut i = 0u8;
        let mut out = EdgeCycles::default();
        let mut v = Vec::<Edge>::new();
        while unseen != 0 {
            unseen &= !(1 << i);
            // Follow a cycle, including pieces flipped in place
            if self[i].0 & 0xf != i || self[i].0 & 0x10 != 0 {
                v.insert(0, self[i]);
                i = self[i].0 & 0xf;
                // Keep accumulating until we get back to the cycle start
                if (unseen & (1 << i)) != 0 {
                    continue;
                }
            }
            // Otherwise, find the lowest unseen piece
            i = unseen.trailing_zeros() as u8;
            if v.is_empty() {
                continue;
            }
            // Append a cycle if there is a non-empty one
            out.0.push(v);
            v = Vec::<Edge>::new();
        }
        out
    }
    fn pack(&self) -> u64 {
        let mut out = self[0].0 as u64;
        out |= (self[1].0 as u64) << 5;
        out |= (self[2].0 as u64) << 10;
        out |= (self[3].0 as u64) << 15;
        out |= (self[4].0 as u64) << 20;
        out |= (self[5].0 as u64) << 25;
        out |= (self[6].0 as u64) << 30;
        out |= (self[7].0 as u64) << 35;
        out |= (self[8].0 as u64) << 40;
        out |= (self[9].0 as u64) << 45;
        out |= (self[10].0 as u64) << 50;
        out |= (self[11].0 as u64) << 55;
        out
    }
    fn net_flip(&self) -> u8 {
        self.0.into_iter().map(|x| (x.0 & 0x10) >> 4).sum::<u8>() & 1
    }
}
impl Edges {
    pub fn turns(&self, t: &[Turn]) -> Edges {
        self * t
    }
}

macro_rules! edges {
    ($(($id:expr, $flip:expr)),*) => {
        Edges([
            $( Edge($id | ($flip << 4)), )*
        ])
    }
}
static EDGE_TURNS: [Edges; NTURNS] = edge_turns!();

#[derive(Debug, Default)]
pub struct EdgeCycles(Vec<Vec<Edge>>);
impl EdgeCyclesTrait for EdgeCycles {
    fn speffz(&self) -> String {
        let mut out = String::new();
        for cycle in &self.0 {
            let mut flip = 0u8;
            let s: String = cycle.iter().map(|x| {
                let pflip = flip;
                flip ^= x.0 & 0x10;
                Edge((x.0 & 0xf) | pflip).speffz()
            }).collect();
            let f = match flip {
                0x10 => "+",
                _ => "",
            };
            out += &format!("({}){}", s, f);
        }
        out
    }
}
impl Display for EdgeCycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for cycle in &self.0 {
            let mut flip = 0u8;
            let s: String = cycle.iter().map(|x| {
                let pflip = flip;
                flip ^= x.0 & 0x10;
                Edge((x.0 &0xf) | pflip)
            }).join(",");
            write!(f, "({s})")?;
            match flip {
                0x10 => { write!(f, "+")?; },
                _ => (),
            };
        }
        Ok(())
    }
}
