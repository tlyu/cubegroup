use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

use bytemuck::*;
use itertools::Itertools;

use super::*;
use crate::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Pod, Zeroable)]
#[repr(transparent)]
pub struct Corners(pub(crate) [Corner; 8]);
const CORNERS_IDENT: Corners = must_cast([0u8, 1, 2, 3, 4, 5, 6, 7]);

impl Default for Corners {
    fn default() -> Self { CORNERS_IDENT }
}
impl Mul for Corners {
    type Output = Corners;
    fn mul(self, rhs: Self) -> Self::Output {
        Corners(rhs.0.map(|x| self[x.id()].dotwist(x.twist())))
    }
}
impl Mul<Turn> for Corners {
    type Output = Corners;
    fn mul(self, rhs: Turn) -> Corners {
        self * CORNER_TURNS[rhs]
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
gen_ops! {
    Corners
}
impl CubeOps for Corners {}
impl CornersTrait for Corners {
    type Cycles = CornerCycles;
    fn parity(&self) -> bool {
        let mut unseen = (1u16 << NCORNERS) - 1;
        let mut i = 0u8;
        let mut out = false;
        while unseen != 0 {
            unseen &= !(1 << i);
            // Follow a cycle
            if self[i].id() != i {
                i = self[i].id();
                // Only toggle parity if not coming back to the cycle start
                if (unseen & (1 << i)) != 0 {
                    out = !out;
                    continue;
                }
            }
            // Otherwise, find the lowest unseen piece
            i = unseen.trailing_zeros() as u8;
        }
        out
    }
    fn cycles(&self) -> CornerCycles {
        let mut unseen = (1u16 << NCORNERS) - 1;
        let mut i = 0u8;
        let mut out = CornerCycles::default();
        let mut v = Vec::<Corner>::new();
        while unseen != 0 {
            unseen &= !(1 << i);
            // Follow a cycle, including pieces twisted in place
            if self[i].id() != i || self[i].twist() != 0 {
                v.insert(0, self[i]);
                i = self[i].id();
                // Keep accumulating until we get back to the cycle start
                if (unseen & (1 << i)) != 0 {
                    continue;
                }
            }
            // Otherwise, find the lowest unseen piece
            i = unseen.trailing_zeros() as u8;
            if v.is_empty() {
                continue;
            }
            // Append a cycle if there is a non-empty one
            out.0.push(v);
            v = Vec::<Corner>::new();
        }
        out
    }
    fn pack(&self) -> u64 {
        let mut out = self[0].0 as u64;
        out |= (self[1].0 as u64) << 5;
        out |= (self[2].0 as u64) << 10;
        out |= (self[3].0 as u64) << 15;
        out |= (self[4].0 as u64) << 20;
        out |= (self[5].0 as u64) << 25;
        out |= (self[6].0 as u64) << 30;
        out |= (self[7].0 as u64) << 35;
        out
    }
    fn net_twist(&self) -> u8 {
        self.0.into_iter().map(|x| x.twist()).sum::<u8>() % 3
    }
    fn co(&self) -> u16 {
        self.0.iter().take(NCORNERS-1).enumerate()
            .map(|(i, c)| c.twist() as u16 * 3u16.pow(i as u32))
            .sum::<u16>()
    }
    fn set_co(co: u16) -> Self {
        let mut out = Corners::default();
        let mut net_twist = 0u8;
        for (i, c) in out.0.iter_mut().take(NCORNERS-1).enumerate() {
            let twist = ((co / 3u16.pow(i as u32)) % 3) as u8;
            *c = c.dotwist(twist as u8);
            net_twist += twist;
        }
        net_twist %= 3;
        out.0[NCORNERS-1] = out.0[NCORNERS-1].dotwist((3-net_twist)%3);
        out
    }
}
impl Corners {
    fn new () -> Self { Corners([Corner(0); 8]) }
    pub fn turns(&self, t: &[Turn]) -> Self {
        self * t
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
            $( Corner($id | ($tw << 3)), )*
        ])
    }
}

static CORNER_TURNS: [Corners; NTURNS] = corner_turns!();

#[derive(Debug, Default)]
pub struct CornerCycles(Vec<Vec<Corner>>);
impl CornerCyclesTrait for CornerCycles {
    fn speffz(&self) -> String {
        let mut out = String::new();
        for cycle in &self.0 {
            let mut twist = 0u8;
            let s: String = cycle.iter().map(|x| {
                let ptwist = twist;
                twist = (twist + 3 - x.twist()) % 3;
                Corner(x.id()).dotwist(ptwist).speffz()
            }).collect();
            out += &format!("({})", s);
            match twist {
                2 => out += "+",
                1 => out += "-",
                _ => ()
            };
        }
        out
    }
}
impl Display for CornerCycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for cycle in &self.0 {
            let mut twist = 0u8;
            let s = cycle.iter().map(|x| {
                let ptwist = twist;
                twist = (twist + 3 - x.twist()) % 3;
                Corner(x.id()).dotwist(ptwist)
            }).join(",");
            write!(f, "({s})")?;
            match twist {
                2 => write!(f, "+")?,
                1 => write!(f, "-")?,
                _ => (),
            };
        }
        Ok(())
    }
}
