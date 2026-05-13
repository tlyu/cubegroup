#![cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::*;
use std::ops::{Mul, Not};

use crate::{Turn, Turns};
use super::*;

union Load8x16 {
    a: uint8x16_t,
    b: [u8; 16],
    qq: u128,
}
const EP_MASK: uint8x16_t = unsafe { Load8x16 { qq: 0x0f0f0f0f0f0f0f0f0f0f0f0f } .a };
const EO_MASK: uint8x16_t = unsafe { Load8x16 { qq: 0x101010101010101010101010 } .a };
const EDGES_IDENT: uint8x16_t = unsafe { Load8x16 { qq: 0x0b0a09080706050403020100 } .a };

macro_rules! edge {
    ( $perm:expr ) => {
            Edges(unsafe { Load8x16 { qq: $perm } .a } )
    }
}

static EDGE_TURNS: [Edges; 18] = [
    edge!(0x0b0a09080706050402010003), // U1
    edge!(0x0b0a09080706050401000302), // U2
    edge!(0x0b0a09080706050400030201), // U3

    edge!(0x0b0a05080709010403020600), // R1
    edge!(0x0b0a01080705060403020900), // R2
    edge!(0x0b0a06080701090403020500), // R3

    edge!(0x0b0a09161812050403170100), // F1
    edge!(0x0b0a09020607050403080100), // F2
    edge!(0x0b0a09171218050403160100), // F3

    edge!(0x0a09080b0706050403020100), // D1
    edge!(0x09080b0a0706050403020100), // D2
    edge!(0x080b0a090706050403020100), // D3

    edge!(0x060a09080306050b04020100), // L1
    edge!(0x030a0908040605070b020100), // L2
    edge!(0x040a09080b06050307020100), // L3

    edge!(0x0b14090807061a1003020115), // B1
    edge!(0x0b0009080706040503020108), // B2
    edge!(0x0b1509080706101a03020114), // B3
];

#[derive(Clone, Copy, Debug)]
pub struct Edges(uint8x16_t);
impl Mul for Edges {
    type Output = Edges;
    fn mul(self, rhs: Edges) -> Edges {
        let mut out = unsafe { vqtbl1q_u8(self.0, vandq_u8(rhs.0, EP_MASK)) };
        out = unsafe { veorq_u8(out, vandq_u8(rhs.0, EO_MASK)) };
        Edges(out)
    }
}
impl Mul<&Edges> for &Edges {
    type Output = Edges;
    fn mul(self, rhs: &Edges) -> Edges {
        *self * *rhs
    }
}
impl Mul<Turn> for Edges {
    type Output = Edges;
    fn mul(self, rhs: Turn) -> Edges {
        self * EDGE_TURNS[rhs]
    }
}
impl Mul<Turn> for &Edges {
    type Output = Edges;
    fn mul(self, rhs: Turn) -> Edges {
        *self * rhs
    }
}
impl Not for Edges {
    type Output = Edges;
    fn not(self) -> Edges {
        let mut out = Load8x16 { qq: 0u128 };
        let a = Load8x16 { a: self.0 };
        for i in 0..NEDGES {
            let v = unsafe { a.b[i] };
            unsafe { out.b[v as usize & 0xf] = i as u8 | v & 0x10 };
        }
        Edges(unsafe { out.a })
    }
}
