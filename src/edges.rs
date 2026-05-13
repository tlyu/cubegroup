use std::hash::Hash;
use std::fmt::{Debug, Display};
use std::ops::{Mul, Not};

use crate::{Turn, Turns};

mod edges_array;
mod edges_neon;

pub use edges_array::*;

const NEDGES: usize = 12;

pub trait EdgesTrait<T: EdgeCyclesTrait>
    where Self: Clone + Copy + Debug + Default + Display
        + Eq + Hash + Mul + Mul<Turn> + Not + PartialEq + Sized,
        for<'a> &'a Self: Mul<&'a Self>,
        for<'a> &'a Self: Mul<Turn>,
        for<'a> &'a Self: Mul<&'a Turns>
{
    fn parity(&self) -> bool;
    fn cycles(&self) -> T;
}
pub trait EdgeCyclesTrait: Debug + Display {}
