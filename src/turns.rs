use std::fmt::{self, Display};
use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Face {
    U, R, F, D, L, B,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Axis {
    Y, X, Z,
}
#[allow(unused)]
#[derive(Clone, Copy, Debug)]
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
    fn filter(self, rhs: Turn) -> bool {
        if self.face() == rhs.face() {
            return false;
        }
        !self.face_sign() || self.axis() != rhs.axis()
    }
}
pub struct TurnIter(Box<dyn Iterator<Item=Turn>>);
impl TurnIter {
    fn new(turn: Turn) -> Self {
        Self (
            Box::new((0..18)
                .map(|x| unsafe { std::mem::transmute::<_, Turn>(x as u8)})
                .filter(move |x| turn.filter(*x)))
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
    fn into_iter(self) -> Self::IntoIter {
        TurnIter::new(self)
    }
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
impl Display for Turns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut first = true;
        for t in &self.0 {
            if !first { write!(f, " ")?; }
            first = false;
            write!(f, "{t}")?;
        }
        Ok(())
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
