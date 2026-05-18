use super::*;
use super::edges_array::*;
use super::edges_neon;

const SPEFFZ_EDGES: [&str; NFLIP] = [
    "ABCDRTJLUVWX",
    "QMIEHNPFKOSG"
];

pub trait SpeffzLetter: Sized {
    fn from_speffz(c: char) -> Result<Self, ()>;
    fn speffz(self) -> char;
}

pub trait Speffz: Sized {
    fn from_speffz<T: AsRef<str>>(s: T) -> Result<Self, ()>;
    fn speffz(self) -> String;
}

impl SpeffzLetter for Edge {
    fn from_speffz(c: char) -> Result<Self, ()> {
        for flip in 0..2 {
            let Some(id) = SPEFFZ_EDGES[flip].find(c) else { continue; };
            return Ok(Edge((id | (flip << 4)) as u8));
        }
        Err(())        
    }
    fn speffz(self) -> char {
        SPEFFZ_EDGES[(self.0 as usize) >> 4].as_bytes()[self.0 as usize & 0xf] as char        
        
    }
}

impl Speffz for edges_array::Edges {
    fn from_speffz<T: AsRef<str>>(s: T) -> Result<Self, ()> {
        let s = s.as_ref();
        let r: Result<Vec<_>, ()> = s.chars().map(|c| Edge::from_speffz(c)).collect();
        let out: [Edge; NEDGES] = r?[..].try_into().map_err(|_| ())?;
        Ok(Edges(out))
    }
    fn speffz(self) -> String {
        self.0.into_iter().map(Edge::speffz).collect()
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
impl Speffz for edges_neon::Edges {
    fn from_speffz<T: AsRef<str>>(s: T) -> Result<Self, ()> {
        Ok(edges_array::Edges::from_speffz(s)?.into())
    }
    fn speffz(self) -> String {
        edges_array::Edges::from(self).speffz()
    }
}
