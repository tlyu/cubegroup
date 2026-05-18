use std::fmt::{Debug, Display};
use std::ops::{Mul, Not};

pub mod corners_array;
pub mod corners_neon;
use crate::{Turn, Turns};

const NCORNERS: usize = 8;

const SPEFFZ_CORNERS: [&str; 3] = [
    "ABCDUVWX",
    "RNJFLPTH",
    "EQMIGKOS",
];

pub trait CornersOps
    where Self: Sized + Mul + Mul<Turn> + Not
        + Eq + Ord + PartialEq + PartialOrd,
        for<'a> Self: Mul<&'a Turns>
{
}
pub trait CornersTrait
    where Self: Clone + Copy + Debug + Display
        + CornersOps
{
    type Cycles: CornerCyclesTrait;
    fn parity(&self) -> bool;
    fn cycles(&self) -> Self::Cycles;
    fn pack(&self) -> u64;
    fn speffz(self) -> String;
    fn net_twist(&self) -> u8;
    fn from_speffz(s: &str) -> Result<Self, ()>;
}
pub trait CornerCyclesTrait: Debug + Display {
    fn speffz(&self) -> String;
}

macro_rules! corner_turns {
    () => {
        [
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

            corners![(7, 2), (1, 0), (2, 0), (0, 1), (3, 2), (5, 0), (6, 0), (4, 1)], // L1
            corners![(4, 0), (1, 0), (2, 0), (7, 0), (0, 0), (5, 0), (6, 0), (3, 0)], // L2
            corners![(3, 2), (1, 0), (2, 0), (4, 1), (7, 2), (5, 0), (6, 0), (0, 1)], // L3

            corners![(1, 1), (6, 2), (2, 0), (3, 0), (4, 0), (5, 0), (7, 1), (0, 2)], // B1
            corners![(6, 0), (7, 0), (2, 0), (3, 0), (4, 0), (5, 0), (0, 0), (1, 0)], // B2
            corners![(7, 1), (0, 2), (2, 0), (3, 0), (4, 0), (5, 0), (1, 1), (6, 2)], // B3
        ]
    }
}
pub(crate) use corner_turns;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_speffz() {
        assert_eq!(corners_array::Corners::from_speffz("ABCDUVWX").unwrap(), corners_array::Corners::default());
        assert_eq!(corners_neon::Corners::from_speffz("ABCDUVWX").unwrap(), corners_neon::Corners::default());
        assert_ne!(corners_array::Corners::from_speffz("BCDAUVWX").unwrap(), corners_array::Corners::default());
        assert_ne!(corners_neon::Corners::from_speffz("BCDAUVWX").unwrap(), corners_neon::Corners::default());
    }
}
