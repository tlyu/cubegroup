use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};

use bytemuck::*;
use itertools::Itertools;

use super::*;
use crate::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Pod, Zeroable)]
#[repr(transparent)]
pub struct Corner(pub(crate) u8);
impl Corner {
    pub(crate) fn id(&self) -> u8 { self.0 & 0x07 }
    pub(crate) fn twist(&self) -> u8 { (self.0 & 0x18) >> 3 }
    fn dotwist(&self, t: u8) -> Self {
        Corner(self.id() | (((self.twist() + t) % 3) << 3))
    }
    fn untwist(&self, t: u8) -> Self { self.dotwist(3 - t) }
    pub fn speffz(self) -> char {
        SPEFFZ_CORNERS[self.twist() as usize].as_bytes()[self.id() as usize] as char
    }
    pub fn from_speffz(c: char) -> Result<Self, ()> {
        for twist in 0..3 {
            let Some(id) = SPEFFZ_CORNERS[twist].find(c) else { continue; };
            return Ok(Corner((id | (twist << 3)) as u8));
        }
        Err(())
    }
}
impl From<u8> for Corner {
    fn from(id: u8) -> Corner { Corner(id) }
}
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
        let mut unseen = 0xffu8;
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
        let mut unseen = 0xffu8;
        let mut i = 0u8;
        let mut out = CornerCycles::default();
        let mut v = Vec::<Corner>::new();
        let mut twist = 0u8;
        while unseen != 0 {
            unseen &= !(1 << i);
            // Follow a cycle, including pieces twisted in place
            if self[i].id() != i || self[i].twist() != 0 {
                twist = (twist + 3 - self[i].twist()) % 3;
                i = self[i].id();
                v.insert(0, Corner(i).untwist(twist));
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
            out.0.push((v, twist));
            twist = 0;
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
    fn speffz(self) -> String {
        self.0.into_iter().map(Corner::speffz).collect()
    }
    fn net_twist(&self) -> u8 {
        self.0.into_iter().map(|x| x.twist()).sum::<u8>() % 3
    }
    fn from_speffz(s: &str) -> Result<Self, ()> {
        let r: Result<Vec<_>, ()> = s.chars().map(|c| Corner::from_speffz(c)).collect();
        let out: [Corner; NCORNERS] = r?[..].try_into().map_err(|_| ())?;
        Ok(Corners(out))
    }
}
impl Corners {
    fn new () -> Self { Corners([Corner(0); 8]) }
    pub fn turns(&self, t: &[Turn]) -> Self {
        let mut out = *self;
        for x in t {
            out = out * *x;
        }
        out
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
pub struct CornerCycles(Vec<(Vec<Corner>, u8)>);
impl CornerCyclesTrait for CornerCycles {
    fn speffz(&self) -> String {
        let mut out = String::new();
        for (c, twist) in &self.0 {
            let s: String = c.iter().map(|x| x.speffz()).collect();
            let t = match twist % 3{
                1 => "-",
                2 => "+",
                _  => "",
            };
            out += &format!("({}){}", s, t);
        }
        out
    }
}
impl Display for CornerCycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for (c, twist) in &self.0 {
            let s = c.iter().join(",");
            write!(f, "({s})")?;
            match twist % 3 {
                1 => { write!(f, "-")?; },
                2 => { write!(f, "+")?; },
                _ => (),
            };
        }
        Ok(())
    }
}
