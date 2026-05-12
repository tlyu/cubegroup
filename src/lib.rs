use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Mul, Not};
use std::str::FromStr;

mod corners;
pub use corners::*;
mod edges;
pub use edges::*;

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
pub struct Turns(Vec<Turn>);
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
        let r = s.split_whitespace().skip_while(|x| x.is_empty())
            .map(Turn::from_str).collect::<Result<Vec<_>, _>>().or(Err(()));
        Ok(Turns(r?))
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
