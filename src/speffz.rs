use std::array::TryFromSliceError;

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

const SPEFFZ_CORNERS: [&str; 3] = [
    "ABCDUVWX",
    "RNJFLPTH",
    "EQMIGKOS",
];

const SPEFFZ_EDGES: [&str; NFLIP] = [
    "ABCDRTJLUVWX",
    "QMIEHNPFKOSG"
];


impl SpeffzLetter for Corner {
    fn speffz(self) -> char {
        SPEFFZ_CORNERS[self.twist() as usize].as_bytes()[self.id() as usize] as char
    }
    fn from_speffz(c: char) -> Result<Self, ParseSpeffzError> {
        for twist in 0..3 {
            let Some(id) = SPEFFZ_CORNERS[twist].find(c) else { continue; };
            return Ok(Corner((id | (twist << 3)) as u8));
        }
        Err(ParseSpeffzError::BadSpeffzLetter)
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

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
impl Speffz for corners_neon::Corners {
    fn speffz(self) -> String {
        corners_array::Corners::from(self).speffz()
    }
    fn from_speffz(s: &str) -> Result<Self, ParseSpeffzError> {
        Ok(corners_array::Corners::from_speffz(s)?.into())
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
        for flip in 0..2 {
            let Some(id) = SPEFFZ_EDGES[flip].find(c) else { continue; };
            return Ok(Edge((id | (flip << 4)) as u8));
        }
        Err(ParseSpeffzError::BadSpeffzLetter)
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
impl Speffz for edges_neon::Edges {
    fn from_speffz(s: &str) -> Result<Self, ParseSpeffzError> {
        Ok(edges_array::Edges::from_speffz(s)?.into())
    }
    fn speffz(self) -> String {
        edges_array::Edges::from(self).speffz()
    }
}
