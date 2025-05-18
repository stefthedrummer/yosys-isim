#![allow(non_camel_case_types)]

pub const SIMD_WIDTH: usize = 8;

pub type simd_int = u32;

pub type Simd<T = simd_int> = std::simd::Simd<T, { SIMD_WIDTH }>;
pub type Mask<T = i32> = std::simd::Mask<T, { SIMD_WIDTH }>;
