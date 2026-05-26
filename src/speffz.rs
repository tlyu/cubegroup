use std::array::TryFromSliceError;

use bytemuck::*;
use thiserror::Error;

use super::corners::*;
use super::edges::*;

#[derive(Debug, Error)]
pub enum ParseSpeffzError {
    #[error("invalid Speffz letter")]
    BadSpeffzLetter,
    #[error("slice conversion error {}", 0)]
    SliceError(#[from] TryFromSliceError),
    #[error("misc Speffz parse error")]
    MiscParseError,
}

const SPEFFZ_CORNERS: [[u8; NCORNERS]; NTWIST] = [
    *b"ABCDUVWX",
    *b"RNJFLPTH",
    *b"EQMIGKOS",
];

const SPEFFZ_EDGES: [&str; NFLIP] = [
    "ABCDRTJLUVWX",
    "QMIEHNPFKOSG"
];

const CORNERS_FROM_SPEFFZ: [u8; NCORNERS * NTWIST] = [
    0x00, 0x01, 0x02, 0x03, 0x10, 0x0b, 0x14, 0x0f,
    0x13, 0x0a, 0x15, 0x0c, 0x12, 0x09, 0x16, 0x0d,
    0x11, 0x08, 0x17, 0x0e, 0x04, 0x05, 0x06, 0x07,
];

const EDGES_FROM_SPEFFZ: [u8; NEDGES * NFLIP] = [
    0x00, 0x01, 0x02, 0x03, 0x13, 0x17, 0x1b, 0x14,
    0x12, 0x06, 0x18, 0x07, 0x11, 0x15, 0x19, 0x16,
    0x10, 0x04, 0x1a, 0x05, 0x08, 0x09, 0x0a, 0x0b,
];

impl SpeffzLetter for Corner {
    fn speffz(self) -> char {
        SPEFFZ_CORNERS[self.twist() as usize][self.id() as usize] as char
    }
    fn from_speffz(c: char) -> Result<Self, ParseSpeffzError> {
        match c as u8 {
            c @ b'A'..=b'X' => {
                Ok(CORNERS_FROM_SPEFFZ[(c - b'A') as usize].into())
            },
            _ => Err(ParseSpeffzError::BadSpeffzLetter)
        }
    }
}

impl Speffz for corners_array::Corners {
    fn speffz(self) -> String {
        self.0.into_iter().map(Corner::speffz).collect()
    }
    fn from_speffz(s: &str) -> Result<Self, ParseSpeffzError> {
        let r: Result<Vec<_>, _> = s.chars().map(|c| Corner::from_speffz(c)).collect();
        let out: [Corner; NCORNERS] = r?[..].try_into()?;
        Ok(corners_array::Corners(out))
    }
}

pub trait SpeffzLetter: Sized {
    fn from_speffz(c: char) -> Result<Self, ParseSpeffzError>;
    fn speffz(self) -> char;
}

pub trait Speffz: Sized {
    fn from_speffz(s: &str) -> Result<Self, ParseSpeffzError>;
    fn speffz(self) -> String;
}

impl SpeffzLetter for Edge {
    fn from_speffz(c: char) -> Result<Self, ParseSpeffzError> {
        match c as u8 {
            c @ b'A'..=b'X' => {
                Ok(Edge(EDGES_FROM_SPEFFZ[(c - b'A') as usize].into()))
            },
            _ => Err(ParseSpeffzError::BadSpeffzLetter),
        }
    }
    fn speffz(self) -> char {
        SPEFFZ_EDGES[(self.0 as usize) >> 4].as_bytes()[self.0 as usize & 0xf] as char
    }
}

impl Speffz for edges_array::Edges {
    fn from_speffz(s: &str) -> Result<Self, ParseSpeffzError> {
        let r: Result<Vec<_>, _> = s.chars().map(|c| Edge::from_speffz(c)).collect();
        let out: [Edge; NEDGES] = r?[..].try_into()?;
        Ok(edges_array::Edges(out))
    }
    fn speffz(self) -> String {
        self.0.into_iter().map(Edge::speffz).collect()
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod neon {
    use std::arch::aarch64::*;
    use super::*;

    impl Speffz for corners_neon::Corners {
        fn speffz(self) -> String {
            let t = must_cast(SPEFFZ_CORNERS);
            let s: [u8; 8] = must_cast(unsafe { vtbl3_u8(t, self.0) });
            unsafe { String::from_utf8_unchecked(s.into()) }
        }
        fn from_speffz(s: &str) -> Result<Self, ParseSpeffzError> {
            let a: [u8; NCORNERS] = s.as_bytes().try_into()?;
            let idx = unsafe { vsub_u8(must_cast(a), must_cast([b'A'; 8])) };
            let err = unsafe { vmaxv_u8(idx) };
            let out = unsafe { vtbl3_u8(must_cast(CORNERS_FROM_SPEFFZ), idx) };
            match err {
                0..24 => Ok(must_cast(out)),
                _ => Err(ParseSpeffzError::BadSpeffzLetter)
            }
        }
    }

    impl Speffz for edges_neon::Edges {
        fn from_speffz(s: &str) -> Result<Self, ParseSpeffzError> {
            Ok(edges_array::Edges::from_speffz(s)?.into())
        }
        fn speffz(self) -> String {
            edges_array::Edges::from(self).speffz()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speffz_roundtrip() {
        for s in 'A'..='X' {
            let c = Corner::from_speffz(s).unwrap();
            let e = Edge::from_speffz(s).unwrap();
            assert_eq!(s, c.speffz());
            assert_eq!(s, e.speffz());
        }
    }
}
