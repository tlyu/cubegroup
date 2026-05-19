use std::fmt::{self, Display};

use crate::edges::*;

impl Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let (id, flip) = (self.0 as usize & 0x0f, (self.0 as usize & 0x10) >> 4);
        write!(f, "{}", EDGES_SINGMASTER[flip][id])
    }
}

impl Display for edges_array::Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s = self.0.map(|x| x.to_string()).join(" ");
        write!(f, "{s}")
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
impl Display for edges_neon::Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", edges_array::Edges::from(*self))
    }
}
