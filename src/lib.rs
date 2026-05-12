use std::fmt::{self, Display};
use std::ops::{Mul, Not};

mod corners;
pub use corners::*;
mod edges;
pub use edges::*;
mod turns;
pub use turns::*;

pub trait CornersTrait: Clone + Copy + fmt::Debug + Display + Eq + Mul + Mul<Turn> + for<'a> Mul<&'a Turns> + Not + PartialEq + Sized
    where for<'a> &'a Self: Mul<&'a Self>, for<'a> &'a Self: Mul<Turn>, for<'a> &'a Self: Mul<&'a Turns>
{
}
impl CornersTrait for Corners {}
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
impl Mul<&Turns> for Cube {
    type Output = Cube;
    fn mul(self, rhs: &Turns) -> Cube {
        Cube(self.0 * rhs, self.1 * rhs)
    }
}
impl Mul<&Turns> for &Cube {
    type Output = Cube;
    fn mul(self, rhs: &Turns) -> Cube {
        *self * rhs
    }
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
    pub fn cycles(&self) -> CubeCycles {
        CubeCycles(self.0.cycles(), self.1.cycles())
    }
}

#[derive(Debug, Default)]
pub struct CubeCycles(CornerCycles, EdgeCycles);
impl Display for CubeCycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.0, self.1)
    }
}
