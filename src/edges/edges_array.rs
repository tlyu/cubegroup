use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

use crate::Turn;
use crate::Turns;

use super::*;

// Lower 4 bits for id, bit 4 for flip
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Edge(pub(crate) u8);
impl Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let (id, flip) = (self.0 as usize & 0x0f, (self.0 as usize & 0x10) >> 4);
        let s = EDGES_SINGMASTER[id];
        write!(f, "{}{}", &s[flip..], &s[..flip])
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Edges(pub(crate) [Edge; 12]);
static EDGES_SINGMASTER: [&str; 12] = [
    "UB", "UR", "UF", "UL",
    "BL", "BR", "FR", "FL",
    "DF", "DR", "DB", "DL",
];
const EDGES_IDENT: Edges = const {
    let mut out = Edges([Edge(0); 12]);
    let mut i = 0;
    while i < 12 {
        out.0[i as usize] = Edge(i);
        i += 1;
    }
    out
};
impl Default for Edges {
    fn default() -> Self {
        EDGES_IDENT
    }
}
impl Display for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut first = true;
        for x in self.0 {
            if !first { write!(f, " ")?; }
            first = false;
            write!(f, "{x}")?;
        }
        Ok(())
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
impl<'a> Mul<&'a Edges> for &'a Edges{
    type Output = Edges;
    fn mul(self, rhs: Self) -> Edges {
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
    fn not(self) -> Self {
        let mut out = Edges([Edge(0); 12]);
        for i in 0..12 {
            let e = self[i].0;
            out[e & 0xf] = Edge(i | (e & 0x10));
        }
        out
    }
}
impl Not for &Edges {
    type Output = Edges;
    fn not(self) -> Edges { !*self }
}
impl EdgesTrait<EdgeCycles> for Edges {
    fn parity(&self) -> bool {
        let mut unseen = 0xfffu16;
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
        let mut unseen = 0xfffu16;
        let mut i = 0u8;
        let mut out = EdgeCycles::default();
        let mut v = Vec::<Edge>::new();
        let mut flip = 0u8;
        while unseen != 0 {
            unseen &= !(1 << i);
            // Follow a cycle, including pieces twisted in place
            if self[i].0 & 0xf != i || self[i].0 & 0x10 != 0 {
                flip ^= self[i].0 & 0x10;
                i = self[i].0 & 0xf;
                v.insert(0, Edge(i | flip));
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
            out.0.push((v, flip));
            flip = 0;
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
}
impl Edges {
    pub fn turns(&self, t: &[Turn]) -> Edges {
        let mut out = *self;
        for x in t {
            out = out * *x;
        }
        out
    }
}

macro_rules! edges {
    ($(($id:expr, $flip:expr)),*) => {
        Edges([
            $( Edge($id | ($flip << 4)), )*
        ])
    }
}
static EDGE_TURNS: [Edges; 18] = edge_turns!();

#[derive(Debug, Default)]
pub struct EdgeCycles(Vec<(Vec<Edge>, u8)>);
impl EdgeCyclesTrait for EdgeCycles {}
impl Display for EdgeCycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut first = true;
        for (c, flip) in &self.0 {
            write!(f, "(")?;
            for x in c {
                if !first { write!(f, ",")?; }
                first = false;
                write!(f, "{x}")?;
            }
            write!(f, ")")?;
            match flip {
                x if *x != 0 => { write!(f, "+")?; },
                _ => (),
            };
            first = true;
        }
        Ok(())
    }
}
