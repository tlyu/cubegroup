use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

mod corners;
pub use corners::*;
mod edges;
pub use edges::*;

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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Cube(Corners, Edges);
impl Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}; {}", self.0, self.1)
    }
}
impl Mul for Cube {
    type Output = Cube;
    fn mul(self, rhs: Cube) -> Cube {
        Cube(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl Mul<&Cube> for &Cube {
    type Output = Cube;
    fn mul(self, rhs: &Cube) -> Cube { *self * *rhs }
}
impl Mul<Turn> for Cube {
    type Output = Cube;
    fn mul(self, rhs: Turn) -> Cube {
        Cube(self.0 * rhs, self.1 * rhs)
    }
}
impl Mul<Turn> for &Cube {
    type Output = Cube;
    fn mul(self, rhs: Turn) -> Cube { *self * rhs }
}
impl Not for Cube {
    type Output = Cube;
    fn not(self) -> Cube {
        Cube(!self.0, !self.1)
    }
}
impl Not for &Cube {
    type Output = Cube;
    fn not(self) -> Cube { !*self }
}
impl Cube {
    pub fn turns(&self, t: &[Turn]) -> Cube {
        let mut out = *self;
        for x in t {
            out = out * *x;
        }
        out
    }
}
