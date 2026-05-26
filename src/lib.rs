use std::fmt::{self, Display};
use std::hash::Hash;
use std::ops::{Mul, Not};

use forward_ref_generic::forward_ref_binop;
use forward_ref_generic::forward_ref_unop;

#[macro_use]
mod internal_macros;

mod corners;
pub use corners::*;
mod edges;
pub use edges::*;
mod turns;
pub use turns::*;
mod speffz;
pub use speffz::*;

#[cfg(all(not(feature = "array"), target_arch = "aarch64", target_feature = "neon"))]
pub use {corners_neon::*, edges_neon::*};
#[cfg(feature = "array")]
pub use {corners_array::*, edges_array::*};

pub trait CubeOps where
        Self: Sized + Mul + Mul<Turn> + for<'b> Mul<&'b Turn> + Not,
        Self: Eq + Hash + PartialEq + PartialOrd + Ord
{
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cube(Corners, Edges);
impl Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.0, self.1)
    }
}
impl Mul for Cube {
    type Output = Cube;
    #[inline]
    fn mul(self, rhs: Cube) -> Cube {
        Cube(self.0 * rhs.0, self.1 * rhs.1)
    }
}
impl Mul<Turn> for Cube {
    type Output = Cube;
    #[inline]
    fn mul(self, rhs: Turn) -> Cube {
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
        self * t
    }
    pub fn cycles(&self) -> CubeCycles {
        CubeCycles(self.0.cycles(), self.1.cycles())
    }
    pub fn pack(&self) -> u128 {
        self.0.pack() as u128 | (self.1.pack() as u128) << 40
    }
    pub fn parity(&self) -> bool {
        #[cfg(debug_assertions)]
        {
            let (cp, ep) = (self.0.parity(), self.1.parity());
            debug_assert_eq!(cp, ep);
            cp
        }
        #[cfg(not(debug_assertions))]
        self.0.parity()
    }
    pub fn speffz(self) -> String {
        self.0.speffz() + "." + &self.1.speffz()
    }
    pub fn net_twist(&self) -> u8 {
        self.0.net_twist()
    }
    pub fn net_flip(&self) -> u8 {
        self.1.net_flip()
    }
    pub fn valid(&self) -> bool {
        self.0.net_twist() == 0 &&
            self.1.net_flip() == 0 &&
            self.0.parity() == self.1.parity()
    }
    pub fn from_speffz(s: &str) -> Result<Self, ParseSpeffzError> {
        let mut v = s.split('.');
        let Some(c) = v.next() else { return Err(ParseSpeffzError::MiscParseError) };
        let Some(e) = v.next() else { return Err(ParseSpeffzError::MiscParseError) };
        Ok(Cube(Corners::from_speffz(c)?, Edges::from_speffz(e)?))
    }
}

#[derive(Debug, Default)]
pub struct CubeCycles(<Corners as CornersTrait>::Cycles, <Edges as EdgesTrait>::Cycles);
impl CubeCycles {
    pub fn speffz(&self) -> String {
        self.0.speffz() + "." + &self.1.speffz()
    }
}
impl Display for CubeCycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.0, self.1)
    }
}

gen_ops!{
    Cube
}

#[cfg(test)]
mod tests {
    use super::*;
    use turns::Turn::*;

    #[test]
    fn test_mul() {
        let corners = corners_neon::Corners::default();
        let s = corners * &Turns::from(&[R1, U1, R3, U3]);
        assert_eq!(s * s, corners * &Turns::from(&[R1, U1, R3, U3, R1, U1, R3, U3]));
    }

    #[test]
    fn test_inverses() {
        let corners = corners_neon::Corners::default();
        let edges = edges_neon::Edges::default();
        let cube = Cube::default();
        for (lhs, rhs) in [
            (U1, U3), (U2, U2), (U3, U1),
            (R1, R3), (R2, R2), (R3, R1),
            (F1, F3), (F2, F2), (F3, F1),
            (D1, D3), (D2, D2), (D3, D1),
            (B1, B3), (B2, B2), (B3, B1),
            (L1, L3), (L2, L2), (L3, L1),
        ] {
            assert_eq!(corners * lhs, !(corners * rhs));
            assert_eq!(edges * lhs, !(edges * rhs));
            assert_eq!(cube * lhs, !(cube * rhs));
        }
        for t in allturns() {
            assert_eq!(!!(cube * t), cube *t);
        }
    }

    #[test]
    fn test_parity() {
        let corners = corners_neon::Corners::default();
        let edges = edges_neon::Edges::default();
        let cube = Cube::default();
        for (lhs, rhs) in [
            (U1, true), (U2, false), (U3, true),
            (R1, true), (R2, false), (R3, true),
            (F1, true), (F2, false), (F3, true),
            (D1, true), (D2, false), (D3, true),
            (B1, true), (B2, false), (B3, true),
            (L1, true), (L2, false), (L3, true),
            ] {
                assert_eq!((corners * lhs).parity(), rhs);
                assert_eq!((edges * lhs).parity(), rhs);
                assert_eq!((cube * lhs).parity(), rhs);
        }
        assert!(!(cube * &Turns::from(&[R3, F1, R1, F3])).parity());
    }

    #[test]
    fn test_twist() {
        let c_array = corners_array::Corners::default();
        let c_neon = corners_neon::Corners::default();
        for t in turns::allturns() {
            assert_eq!((c_array * t).net_twist(), 0);
            assert_eq!((c_neon * t).net_twist(), 0);
        }
    }
    #[test]
    fn test_flip() {
        let e_array = edges_array::Edges::default();
        let e_neon = edges_neon::Edges::default();
        for t in turns::allturns() {
            assert_eq!((e_array * t).net_flip(), 0);
            assert_eq!((e_neon * t).net_flip(), 0);
        }
    }
    #[test]
    fn test_valid() {
        let cube = Cube::default();
        for t in turns::allturns() {
            assert!((cube * t).valid())
        }
    }
}
