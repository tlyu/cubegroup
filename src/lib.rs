use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Corner {
    id: u8,
    twist: u8 // 0 = none, 1 = CW, 2 = CCW
}
impl Corner {
    fn twist(&self, t: u8) -> Self {
        Corner { id: self.id, twist: (self.twist + t) % 3 }
    }
    fn untwist(&self, t: u8) -> Self {
        let (id, twist) = (self.id, (3 + self.twist - t) % 3);
        Corner { id: id, twist }
    }
}
impl From<u8> for Corner {
    fn from(id: u8) -> Corner { Corner{ id, twist: 0 } }
}
// Singmaster piece notation: facets in clockwise order
static CORNERS_SINGMASTER: [&str; 8] = [
    "ULB", "UBR", "URF", "UFL",
    "DLF", "DFR", "DRB", "DBL"
];
impl Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s = CORNERS_SINGMASTER[self.id as usize];
        let twist = 3 - self.twist as usize;
        // Singmaster piece notation has the U/D facet first,
        // so invert the twist to output in the correct order
        write!(f, "{}{}", &s[(twist)..], &s[..(twist)])
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Corners {
    v: [Corner; 8]
}
const CORNERS_IDENT: Corners = const {
    let mut c = Corners { v: [Corner { id: 0, twist: 0 }; 8] };
    let mut id = 0;
    while id < 8 {
        c.v[id as usize] = Corner { id, twist: 0 };
        id += 1;
    }
    c
};
impl Default for Corners {
    fn default() -> Self { CORNERS_IDENT }
}
impl Mul for Corners {
    type Output = Corners;
    fn mul(self, rhs: Self) -> Self::Output {
        Corners{v: rhs.v.map(|x| self[x.id].twist(x.twist))}
    }
}
impl<'a> Mul<&'a Corners> for &'a Corners {
    type Output = Corners;
    fn mul(self, rhs: Self) -> Corners {
        *self * *rhs
    }
}
impl Not for Corners {
    type Output = Corners;
    fn not(self) -> Self {
        let mut out = Corners::new();
        for i in 0..8 {
            let v = &mut out[self[i].id];
            v.id = i;
            *v = v.untwist(self[i].twist);
        }
        out
    }
}
impl Not for &Corners {
    type Output = Corners;
    fn not(self) -> Corners { !*self }
}
impl Corners {
    fn new () -> Self { Corners{v: [Corner::default(); 8]} }
    pub fn turn(&self, t: Turn) -> Self {
        self * &CORNER_TURNS[t]
    }
    pub fn turns(&self, t: &[Turn]) -> Self {
        let mut out = *self;
        for x in t {
            out = out.turn(*x);
        }
        out
    }
    pub fn invert(&self) -> Self {
        let mut out = Corners::new();
        for i in 0..8 {
            let v = &mut out[self[i].id];
            v.id = i;
            *v = v.untwist(self[i].twist);
        }
        out
    }
}
impl Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut first = true;
        for corner in &self.v {
            if !first { write!(f, " ")?; }
            first = false;
            write!(f, "{corner}")?;
        }
        Ok(())
    }
}
impl Index<u8> for Corners {
    type Output = Corner;
    fn index(&self, i: u8) -> &Self::Output {
        &self.v[i as usize]
    }
}
impl IndexMut<u8> for Corners {
    fn index_mut(&mut self, i: u8) -> &mut Self::Output {
        &mut self.v[i as usize]
    }
}

#[macro_export]
macro_rules! corners {
    ( $(($id:expr, $tw:expr)),* ) => {
        Corners { v: [
            $( Corner { id: $id, twist: $tw }, )*
        ]}
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub enum Turn {
    U1 = 0,
    U2 = 1,
    U3 = 2,
    R1 = 3,
    R2 = 4,
    R3 = 5,
    F1 = 6,
    F2 = 7,
    F3 = 8,
    D1 = 9,
    D2 = 10,
    D3 = 11,
    B1 = 12,
    B2 = 13,
    B3 = 14,
    L1 = 15,
    L2 = 16,
    L3 = 17,
}
impl<T> Index<Turn> for [T] {
    type Output = T;
    fn index(&self, i: Turn) -> &Self::Output {
        &self[i as usize]
    }
}
impl<T> IndexMut<Turn> for [T] {
    fn index_mut(&mut self, i: Turn) -> &mut Self::Output {
        &mut self[i as usize]
    }
}

const CORNER_TURNS: [Corners; 18] = [
    corners![(3, 0), (0, 0), (1, 0), (2, 0), (4, 0), (5, 0), (6, 0), (7, 0)], // U1
    corners![(2, 0), (3, 0), (0, 0), (1, 0), (4, 0), (5, 0), (6, 0), (7, 0)], // U2
    corners![(1, 0), (2, 0), (3, 0), (0, 0), (4, 0), (5, 0), (6, 0), (7, 0)], // U3

    corners![(0, 0), (2, 1), (5, 2), (3, 0), (4, 0), (6, 1), (1, 2), (7, 0)], // R1
    corners![(0, 0), (5, 0), (6, 0), (3, 0), (4, 0), (1, 0), (2, 0), (7, 0)], // R2
    corners![(0, 0), (6, 1), (1, 2), (3, 0), (4, 0), (2, 1), (5, 2), (7, 0)], // R3

    corners![(0, 0), (1, 0), (3, 1), (4, 2), (5, 1), (2, 2), (6, 0), (7, 0)], // F1
    corners![(0, 0), (1, 0), (4, 0), (5, 0), (2, 0), (3, 0), (6, 0), (7, 0)], // F2
    corners![(0, 0), (1, 0), (5, 1), (2, 2), (3, 1), (4, 2), (6, 0), (7, 0)], // F3

    corners![(0, 0), (1, 0), (2, 0), (3, 0), (7, 0), (4, 0), (5, 0), (6, 0)], // D1
    corners![(0, 0), (1, 0), (2, 0), (3, 0), (6, 0), (7, 0), (4, 0), (5, 0)], // D2
    corners![(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4, 0)], // D3

    corners![(7, 1), (0, 2), (2, 0), (3, 0), (4, 0), (5, 0), (1, 1), (6, 2)], // B1
    corners![(6, 0), (7, 0), (2, 0), (3, 0), (4, 0), (5, 0), (0, 0), (1, 0)], // B1
    corners![(1, 1), (6, 2), (2, 0), (3, 0), (4, 0), (5, 0), (7, 1), (0, 2)], // B1

    corners![(7, 2), (1, 0), (2, 0), (0, 1), (3, 2), (5, 0), (6, 0), (4, 1)], // L1
    corners![(4, 0), (1, 0), (2, 0), (7, 0), (0, 0), (5, 0), (6, 0), (3, 0)], // L2
    corners![(3, 2), (1, 0), (2, 0), (4, 1), (7, 2), (5, 0), (6, 0), (0, 1)], // L3
];

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
    pub fn turn(&self, t: Turn) -> Edges {
        *self * EDGE_TURNS[t]
    }
    pub fn turns(&self, t: &[Turn]) -> Edges {
        let mut out = *self;
        for x in t {
            out = out.turn(*x);
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
