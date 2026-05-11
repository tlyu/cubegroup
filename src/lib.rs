use std::ops::{Index, IndexMut};

mod corners;
pub use corners::*;
mod edges;
pub use edges::*;

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
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

// Lower 4 bits for id, bit 4 for flip
