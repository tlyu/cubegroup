use std::fmt::{self, Display};
use std::iter::{IntoIterator, Iterator};
use std::ops::{Deref, Index, IndexMut};
use std::str::FromStr;

use itertools::Itertools;

pub(crate) const NTURNS: usize = 18;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Face {
    U, R, F, D, L, B,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Axis {
    Y, X, Z,
}

/// An outer turn of a 3x3x3 cube.
///
/// U1 = U, U3 = U', etc.
///
/// This representation embeds the concept of a *canonical sequence* of moves.
/// Canonical sequences are useful for computerized solving.
///
/// Each variant can be iterated over, to produce possible next moves in a
/// canonical sequence starting from that variant.
///
/// ```rust
/// # use cubegroup::Turn;
/// let x: Vec<_> = Turn::U1.into_iter().collect();
/// ```
///
/// The [`allturns()`] function produces an iterator over all moves.
///
/// The [`canonturns()`] function produces an iterator over canonical
/// moves that can follow the provided optional move, or an iterator over all
/// moves, if `None`.
///
/// Two consecutive turns of the same face are equivalent to at most one turn.
/// Two consecutive turns of two faces on the same axis commute with each
/// other.
/// To avoid multiple equivalent sequences, we enforce a priority of moves on
/// each axis.
///
/// The numbering of the turns follows these rules:
/// * Turns of the same face are grouped together
/// * The clockwise quarter turn is first
/// * Turns of opposite faces are in opposite halves of the number space
#[allow(unused)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
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
    L1 = 12,
    L2 = 13,
    L3 = 14,
    B1 = 15,
    B2 = 16,
    B3 = 17,
}
impl Turn {
    fn axis(self) -> Axis {
        match self.face() as u8 % 3 {
            0 => Axis::Y,
            1 => Axis::X,
            2 => Axis::Z,
            _ => unreachable!(),
        }
    }
    fn face(self) -> Face {
        match self as u8 / 3 {
            0 => Face::U,
            1 => Face::R,
            2 => Face::F,
            3 => Face::D,
            4 => Face::L,
            5 => Face::B,
            _ => unreachable!(),
        }
    }
    fn face_sign(self) -> bool {
        (self as u8 / 9) != 0
    }
    // Return `true` if `self` is an allowed next move following `prev`
    fn optfilter(self, prev: Option<Turn>) -> bool {
        let Some(prev) = prev else { return true };
        if self.face() == prev.face() {
            return false;
        }
        !prev.face_sign() || prev.axis() != self.axis()
    }
}

/// Iterate over possible turns.
pub struct TurnIter(Box<dyn Iterator<Item=Turn>>);
impl TurnIter {
    fn new(t: Option<Turn>) -> Self {
        Self(
            Box::new((0..NTURNS)
                .map(|x| unsafe { std::mem::transmute::<_, Turn>(x as u8) })
                .filter(move |x| x.optfilter(t)))
        )
    }
}
impl Iterator for TurnIter {
    type Item = Turn;
    fn next(&mut self) -> Option<Turn> {
        self.0.next()
    }
}

impl IntoIterator for Turn {
    type Item = Turn;
    type IntoIter = TurnIter;
    /// Create an iterator over possible canonical moves, starting from `self`.
    fn into_iter(self) -> Self::IntoIter {
        TurnIter::new(Some(self))
    }
}

/// An iterator over all possible turns
pub fn allturns() -> TurnIter {
    TurnIter::new(None)
}

/// An iterator over all canonical turns that are allowed to follow `prev`.
/// If `prev` is `None`, all turns are allowed.
pub fn canonturns(prev: Option<Turn>) -> TurnIter {
    TurnIter::new(prev)
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
impl Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Turn::*;
        let s = match self {
            U1 => "U",
            U2 => "U2",
            U3 => "U'",
            R1 => "R",
            R2 => "R2",
            R3 => "R'",
            F1 => "F",
            F2 => "F2",
            F3 => "F'",
            D1 => "D",
            D2 => "D2",
            D3 => "D'",
            B1 => "B",
            B2 => "B2",
            B3 => "B'",
            L1 => "L",
            L2 => "L2",
            L3 => "L'",
        };
        write!(f, "{s}")
    }
}
impl FromStr for Turn {
    type Err = ();
    fn from_str(s: &str) -> Result<Turn, ()> {
        use Turn::*;
        let mut out: u8;
        let mut c = s.chars();
        match c.next() {
            Some('U') => { out = U1 as u8; },
            Some('R') => { out = R1 as u8; },
            Some('F') => { out = F1 as u8; },
            Some('D') => { out = D1 as u8; },
            Some('B') => { out = B1 as u8; },
            Some('L') => { out = L1 as u8; },
            _ => { return Err(()); },
        }
        match c.next() {
            Some('\'') | Some('3') => { out += 2; },
            Some('1') => (),
            Some('2') => { out += 1; },
            None => (),
            _ => { return Err(()); },
        }
        // All of the possible values of out are safe at this point
        Ok(unsafe { std::mem::transmute(out) })
    }
}

#[derive(Debug, Default)]
pub struct Turns(pub(crate) Vec<Turn>);
impl Deref for Turns {
    type Target = [Turn];
    fn deref(&self) -> &Self::Target {
        &self.0
    }

}
impl AsRef<[Turn]> for Turns {
    fn as_ref(&self) -> &[Turn] {
        self.deref().as_ref()
    }
}
impl Display for Turns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s = self.0.iter().map(|x| x.to_string()).join(" ");
        write!(f, "{s}")
    }
}
impl FromStr for Turns {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let r = s.split_whitespace().filter(|x| !x.is_empty())
            .map(Turn::from_str).collect::<Result<Vec<_>, _>>().or(Err(()));
        Ok(Turns(r?))
    }
}
impl From<&[Turn]> for Turns {
    fn from(x: &[Turn]) -> Turns {
        Turns(x.into())
    }
}
impl<const N: usize> From<&[Turn; N]> for Turns {
    fn from(x: &[Turn; N]) -> Turns {
        Turns((&x[..]).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Turn::*;

    #[test]
    fn test_turns() {
        let a = [
            ([U1, U2, U3], vec![R1, R2, R3, F1, F2, F3, D1, D2, D3, L1, L2, L3, B1, B2, B3]),
            ([R1, R2, R3], vec![U1, U2, U3, F1, F2, F3, D1, D2, D3, L1, L2, L3, B1, B2, B3]),
            ([F1, F2, F3], vec![U1, U2, U3, R1, R2, R3, D1, D2, D3, L1, L2, L3, B1, B2, B3]),
            ([D1, D2, D3], vec![R1, R2, R3, F1, F2, F3, L1, L2, L3, B1, B2, B3]),
            ([L1, L2, L3], vec![U1, U2, U3, F1, F2, F3, D1, D2, D3, B1, B2, B3]),
            ([B1, B2, B3], vec![U1, U2, U3, R1, R2, R3, D1, D2, D3, L1, L2, L3]),
        ];
        for (turns, seq) in a {
            for t in turns {
                assert_eq!(t.into_iter().collect::<Vec<_>>(), seq);
            }
        }
    }
}
