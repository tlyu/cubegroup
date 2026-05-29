//! Domino reduction coordinates
//!
//! Corner orientation: base 3 number (u16; 12 bits)
//! Edges orientation: base 2 number (u16; 11 bits)
//!
//! Orientation coordinates omit the most significant digit, because the
//! total twist/flip constraints make it redundant.

use std::sync::LazyLock;

use crate::corners::*;
use crate::edges::*;
use crate::Corners;
use crate::Edges;
use crate::turns::*;

pub const NCO: u16 = 2187;
pub const NEO: u16 = 2048;

static CO_TABLE: LazyLock::<COMul> = LazyLock::new(COMul::new);
static EO_TABLE: LazyLock::<EOMul> = LazyLock::new(EOMul::new);

pub struct COMul(Vec<Vec<u16>>);

impl COMul {
    pub fn new() -> Self {
        let mut v = Vec::new();
        for i in 0u16..NCO {
            let row: Vec<_> = allturns().into_iter().map(|t| {
                (Corners::from_co(i) * t).co()
            }).collect();
            v.push(row);
        }
        Self(v)
    }
    #[inline]
    pub fn mul(&self, co: u16, t: Turn) -> u16 {
        self.0[co as usize][t as usize]
    }
}

pub fn init_co_mul() {
    LazyLock::force(&CO_TABLE);
}
#[inline]
pub fn co_mul(co: u16, t: Turn) -> u16 {
    CO_TABLE.mul(co, t)
}

pub struct EOMul(Vec<Vec<u16>>);

impl EOMul {
    pub fn new() -> Self {
        let mut v = Vec::new();
        for i in 0u16..NEO {
            let row: Vec<_> = allturns().into_iter().map(|t| {
                (Edges::from_eo(i) * t).eo()
            }).collect();
            v.push(row);
        }
        Self(v)
    }
    #[inline]
    pub fn mul(&self, eo: u16, t: Turn) -> u16 {
        self.0[eo as usize][t as usize]
    }
}

pub fn init_eo_mul() {
    LazyLock::force(&EO_TABLE);
}
#[inline]
pub fn eo_mul(eo: u16, t: Turn) -> u16 {
    EO_TABLE.mul(eo, t)
}

pub trait COConv {
    fn co(&self) -> u16;
    fn from_co(co: u16) -> Self;
}
pub trait EOConv {
    fn eo(&self) -> u16;
    fn from_eo(eo: u16) -> Self;
}

impl COConv for corners_array::Corners {
    fn co(&self) -> u16 {
        self.0.iter().take(NCORNERS-1).enumerate()
            .map(|(i, c)| c.twist() as u16 * 3u16.pow(i as u32))
            .sum::<u16>()
    }
    fn from_co(co: u16) -> Self {
        let mut out = Self::default();
        let mut net_twist = 0u8;
        for (i, c) in out.0.iter_mut().take(NCORNERS-1).enumerate() {
            let twist = ((co / 3u16.pow(i as u32)) % 3) as u8;
            *c = c.dotwist(twist as u8);
            net_twist += twist;
        }
        net_twist %= 3;
        out.0[NCORNERS-1] = out.0[NCORNERS-1].dotwist((3-net_twist)%3);
        out
    }
}

impl EOConv for edges_array::Edges {
    fn eo(&self) -> u16 {
        let mut out = 0u16;
        for (i, e) in self.0.iter().enumerate() {
            out |= (e.0 as u16 >> 4) << i;
        }
        out & 0x7ff
    }
    fn from_eo(eo: u16) -> Self {
        let eo = eo & 0x7ff;
        let parity = ((eo.count_ones() & 1) << 11)as u16;
        let eo = (eo & 0x7ff) | parity;
        let mut out = Self::default();
        for (i, e) in out.0.iter_mut().enumerate() {
            e.0 |= ((eo >> i) << 4) as u8 & 0x10;
        }
        out
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod neon {
    use std::arch::aarch64::*;
    use bytemuck::*;
    use crate::edges_neon::*;
    use super::*;

    impl COConv for corners_neon::Corners {
        fn co(&self) -> u16 {
            const FACTORS: uint16x8_t = must_cast([
                1u16, 3, 3*3, 3*3*3, 3*3*3*3, 3*3*3*3*3, 3*3*3*3*3*3, 0
            ]);
            let wide = unsafe { vmovl_u8(vshr_n_u8::<3>(self.0)) };
            unsafe { vaddvq_u16(vmulq_u16(wide, FACTORS)) }
        }
        fn from_co(co: u16) -> Self {
            corners_array::Corners::from_co(co).into()
        }
    }

    impl EOConv for edges_neon::Edges {
        fn eo(&self) -> u16 {
            const SHIFTS: int8x8_t = must_cast([0i8, 1, 2, 3, 4, 5, 6, 7]);
            let flips = unsafe { vshrq_n_u8::<4>(self.0) };
            let lo = unsafe { vshl_u8(vget_low_u8(flips), SHIFTS) };
            let hi = unsafe { vshl_u8(vget_high_u8(flips), SHIFTS) };
            let out: u16 = unsafe { vaddv_u8(lo) as u16 };
            (out | unsafe { (vaddv_u8(hi) as u16) << 8 }) & 0x7ff
        }
        fn from_eo(eo: u16) -> Self {
            let eo = eo & 0x7ff;
            let parity = ((eo.count_ones() & 1) << 11) as u16;
            let eo = (eo & 0x7ff) | parity;
            const SHIFTS: int8x8_t = must_cast([0i8, -1, -2, -3, -4, -5, -6, -7]);
            let lo = unsafe { vshl_u8(must_cast([eo as u8; 8]), SHIFTS) };
            let hi = unsafe { vshl_u8(must_cast([(eo >> 8) as u8; 8]), SHIFTS) };
            let flips = unsafe { vandq_u8(vshlq_n_u8::<4>(vcombine_u8(lo, hi)), EO_MASK) };
            Self(unsafe { vorrq_u8(flips, EDGES_IDENT) })
        }
    }
}
