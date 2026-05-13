use std::fmt::{Debug, Display};
use std::ops::{Mul, Not};

mod corners_array;
pub use corners_array::*;
use crate::{Turn, Turns};

pub trait CornersTrait<T: CornerCyclesTrait>
    where Self: Clone + Copy + Debug + Display + Eq + Mul + Mul<Turn>
        + for<'a> Mul<&'a Turns> + Not + PartialEq + Sized,
        for<'a> &'a Self: Mul<&'a Self>,
        for<'a> &'a Self: Mul<Turn>,
        for<'a> &'a Self: Mul<&'a Turns>
{
    fn parity(&self) -> bool;
    fn cycles(&self) -> T;
}
pub trait CornerCyclesTrait: Debug + Display {}
