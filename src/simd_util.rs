#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub union Load8x8 {
    pub a: uint8x8_t,
    pub b: [u8; 8],
    pub q: u64,
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub union Load8x16 {
    pub a: uint8x16_t,
    pub b: [u8; 16],
    pub qq: u128,
}
