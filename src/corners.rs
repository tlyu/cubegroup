use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

use super::Turn;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Corner(u8);
impl Corner {
    fn id(&self) -> u8 { self.0 & 0x07 }
    fn twist(&self) -> u8 { (self.0 & 0x30) >> 4 }
    fn dotwist(&self, t: u8) -> Self {
        Corner(self.id() | (((self.twist() + t) % 3) << 4))
    }
    fn untwist(&self, t: u8) -> Self { self.dotwist(3 - t) }
}
impl From<u8> for Corner {
    fn from(id: u8) -> Corner { Corner(id) }
}
// Singmaster piece notation: facets in clockwise order
static CORNERS_SINGMASTER: [&str; 8] = [
    "ULB", "UBR", "URF", "UFL",
    "DLF", "DFR", "DRB", "DBL"
];
impl Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s = CORNERS_SINGMASTER[self.id() as usize];
        let twist = 3 - self.twist() as usize;
        // Singmaster piece notation has the U/D facet first,
        // so invert the twist to output in the correct order
        write!(f, "{}{}", &s[(twist)..], &s[..(twist)])
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Corners([Corner; 8]);
const CORNERS_IDENT: Corners = const {
    let mut c = Corners([Corner(0); 8]);
    let mut id = 0;
    while id < 8 {
        c.0[id as usize] = Corner(id);
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
        Corners(rhs.0.map(|x| self[x.id()].dotwist(x.twist())))
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
            let v = &mut out[self[i].id()];
            *v = Corner(i);
            *v = v.untwist(self[i].twist());
        }
        out
    }
}
impl Not for &Corners {
    type Output = Corners;
    fn not(self) -> Corners { !*self }
}
impl Corners {
    fn new () -> Self { Corners([Corner(0); 8]) }
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
}
impl Display for Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut first = true;
        for corner in &self.0 {
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
        &self.0[i as usize]
    }
}
impl IndexMut<u8> for Corners {
    fn index_mut(&mut self, i: u8) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

macro_rules! corners {
    ( $(($id:expr, $tw:expr)),* ) => {
        Corners([
            $( Corner($id | ($tw << 4)), )*
        ])
    }
}

static CORNER_TURNS: [Corners; 18] = [
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
