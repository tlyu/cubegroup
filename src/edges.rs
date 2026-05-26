use std::hash::Hash;
use std::fmt::{Debug, Display};

use bytemuck::*;

use crate::speffz::*;
use crate::turns::*;

pub mod edges_array;
pub mod edges_neon;
mod edges_convert;

pub(crate) const NEDGES: usize = 12;
pub(crate) const NFLIP: usize = 2;

static EDGES_SINGMASTER: [[&str; NEDGES]; NFLIP] = [
    [
        "UB", "UR", "UF", "UL",
        "BL", "BR", "FR", "FL",
        "DF", "DR", "DB", "DL",
    ],
    [
        "BU", "RU", "FU", "LU",
        "LB", "RB", "RF", "LF",
        "FD", "RD", "BD", "LD",
    ],
];

// Lower 4 bits for id, bit 4 for flip
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Pod, Zeroable)]
#[repr(transparent)]
pub struct Edge(pub(crate) u8);


pub trait EdgesTrait
    where Self: Clone + Copy + Debug + Default + Display
        + crate::CubeOps + Speffz
{
    type Cycles: EdgeCyclesTrait;
    fn parity(&self) -> bool;
    fn cycles(&self) -> Self::Cycles;
    fn pack(&self) -> u64;
    fn net_flip(&self) -> u8;
    fn eo(&self) -> u16;
    fn set_eo(eo: u16) -> Self;
}
pub trait EdgeCyclesTrait: Debug + Display {
    fn speffz(&self) -> String;
}

macro_rules! edge_turns { () => {[
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

        edges![(0, 0), (1, 0), (2, 0), (4, 0), (11, 0), (5, 0), (6, 0), (3, 0), (8, 0), (9, 0), (10, 0), (7, 0)], // L1
        edges![(0, 0), (1, 0), (2, 0), (11, 0), (7, 0), (5, 0), (6, 0), (4, 0), (8, 0), (9, 0), (10, 0), (3, 0)], // L2
        edges![(0, 0), (1, 0), (2, 0), (7, 0), (3, 0), (5, 0), (6, 0), (11, 0), (8, 0), (9, 0), (10, 0), (4, 0)], // L3

        edges![(5, 1), (1, 0), (2, 0), (3, 0), (0, 1), (10, 1), (6, 0), (7, 0), (8, 0), (9, 0), (4, 1), (11, 0)], // B1
        edges![(10, 0), (1, 0), (2, 0), (3, 0), (5, 0), (4, 0), (6, 0), (7, 0), (8, 0), (9, 0), (0, 0), (11, 0)], // B2
        edges![(4, 1), (1, 0), (2, 0), (3, 0), (10, 1), (0, 1), (6, 0), (7, 0), (8, 0), (9, 0), (5, 1), (11, 0)], // B3
    ]}
}
pub(crate) use edge_turns;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singmaster() {
        for flip in 0..NFLIP {
            for id in 0..NEDGES {
                let s = EDGES_SINGMASTER[flip][id];
                // Make sure that strings are consistent with flips
                assert_eq!(s[flip..].to_string()+&s[..flip], EDGES_SINGMASTER[0][id]);
            }
        }
    }

    #[test]
    fn test_speffz() {
        assert_eq!(edges_array::Edges::from_speffz("ABCDRTJLUVWX").unwrap(), edges_array::Edges::default());
        assert_eq!(edges_neon::Edges::from_speffz("ABCDRTJLUVWX").unwrap(), edges_neon::Edges::default());
        assert_ne!(edges_array::Edges::from_speffz("BCDARTJLUVWX").unwrap(), edges_array::Edges::default());
        assert_ne!(edges_neon::Edges::from_speffz("BCDARTJLUVWX").unwrap(), edges_neon::Edges::default());
    }
}
