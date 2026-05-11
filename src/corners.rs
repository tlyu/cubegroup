use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

use super::Turn;

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
