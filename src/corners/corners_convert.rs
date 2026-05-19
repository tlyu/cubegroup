use std::fmt::{self, Display};

use crate::corners_array;
use crate::corners_neon;
use crate::corners::CORNERS_SINGMASTER;

impl Display for corners_array::Corner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let (id, twist) = (self.id() as usize, self.twist() as usize);
        let s = CORNERS_SINGMASTER[twist][id];
        write!(f, "{s}")
    }
}

impl Display for corners_array::Corners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s = self.0.map(|x| x.to_string()).join(" ");
        write!(f, "{s}")
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
impl Display for corners_neon::Corners {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", corners_array::Corners::from(*self))
    }
}
