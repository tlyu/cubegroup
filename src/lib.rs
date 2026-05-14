use std::fmt::{self, Display};
use std::ops::{Mul, Not};

mod corners;
pub use corners::*;
mod edges;
pub use edges::*;
mod turns;
pub use turns::*;
pub(crate) mod simd_util;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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
impl Mul<Turn> for Cube {
    type Output = Cube;
    fn mul(self, rhs: Turn) -> Cube {
        Cube(self.0 * rhs, self.1 * rhs)
    }
}
impl Mul<&Turns> for Cube {
    type Output = Cube;
    fn mul(self, rhs: &Turns) -> Cube {
        Cube(self.0 * rhs, self.1 * rhs)
    }
}
impl Not for Cube {
    type Output = Cube;
    fn not(self) -> Cube {
        Cube(!self.0, !self.1)
    }
}
impl Cube {
    pub fn turns(&self, t: &[Turn]) -> Cube {
        let mut out = *self;
        for x in t {
            out = out * *x;
        }
        out
    }
    pub fn cycles(&self) -> CubeCycles {
        CubeCycles(self.0.cycles(), self.1.cycles())
    }
    pub fn pack(&self) -> u128 {
        self.0.pack() as u128 | (self.1.pack() as u128) << 40
    }
}

#[derive(Debug, Default)]
pub struct CubeCycles(<Corners as CornersTrait>::Cycles, <Edges as EdgesTrait>::Cycles);
impl Display for CubeCycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.0, self.1)
    }
}
