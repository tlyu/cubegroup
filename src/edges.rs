use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

use super::Turn;

// Lower 4 bits for id, bit 4 for flip
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Edge(u8);
impl Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let (id, flip) = (self.0 as usize & 0x0f, (self.0 as usize & 0x10) >> 4);
        let s = EDGES_SINGMASTER[id];
        write!(f, "{}{}", &s[flip..], &s[..flip])
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Edges([Edge; 12]);
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
static EDGE_TURNS: [Edges; 18] = [
    edges![(3, 0), (0, 0), (1, 0), (2, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0)], // U1
    edges![(2, 0), (3, 0), (0, 0), (1, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0)], // U2
    edges![(1, 0), (2, 0), (3, 0), (0, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0)], // U3

    edges![(0, 0), (6, 0), (2, 0), (3, 0), (4, 0), (1, 0), (9, 0), (7, 0), (8, 0), (5, 0), (10, 0), (11, 0)], // R1
    edges![(0, 0), (9, 0), (2, 0), (3, 0), (4, 0), (6, 0), (5, 0), (7, 0), (8, 0), (1, 0), (10, 0), (11, 0)], // R2
    edges![(0, 0), (5, 0), (2, 0), (3, 0), (4, 0), (9, 0), (1, 0), (7, 0), (8, 0), (6, 0), (10, 0), (11, 0)], // R3

    edges![(0, 0), (1, 0), (7, 1), (3, 0), (4, 0), (5, 0), (2, 1), (8, 1), (6, 1), (9, 0), (10, 0), (11, 0)], // F1
    edges![(0, 0), (1, 0), (8, 0), (3, 0), (4, 0), (5, 0), (7, 0), (6, 0), (2, 0), (9, 0), (10, 0), (11, 0)], // F2
    edges![(0, 0), (1, 0), (6, 1), (3, 0), (4, 0), (5, 0), (8, 1), (2, 1), (7, 1), (9, 0), (10, 0), (11, 0)], // F3

    edges![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (11, 0), (8, 0), (9, 0), (10, 0)], // D1
    edges![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (10, 0), (11, 0), (8, 0), (9, 0)], // D2
    edges![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (9, 0), (10, 0), (11, 0), (8, 0)], // D3

    edges![(5, 1), (1, 0), (2, 0), (3, 0), (0, 1), (10, 1), (6, 0), (7, 0), (8, 0), (9, 0), (4, 1), (11, 0)], // B1
    edges![(10, 0), (1, 0), (2, 0), (3, 0), (5, 0), (4, 0), (6, 0), (7, 0), (8, 0), (9, 0), (0, 0), (11, 0)], // B2
    edges![(4, 1), (1, 0), (2, 0), (3, 0), (10, 1), (0, 1), (6, 0), (7, 0), (8, 0), (9, 0), (5, 1), (11, 0)], // B3

    edges![(0, 0), (1, 0), (2, 0), (4, 0), (11, 0), (5, 0), (6, 0), (3, 0), (8, 0), (9, 0), (10, 0), (7, 0)], // L1
    edges![(0, 0), (1, 0), (2, 0), (11, 0), (7, 0), (5, 0), (6, 0), (4, 0), (8, 0), (9, 0), (10, 0), (3, 0)], // L2
    edges![(0, 0), (1, 0), (2, 0), (7, 0), (3, 0), (5, 0), (6, 0), (11, 0), (8, 0), (9, 0), (10, 0), (4, 0)], // L3
];
