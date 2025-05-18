use crate::__div_ceil;
use crate::Logic;
use crate::LogicSlice;
use crate::logic_int;
use crate::simd::Mask;
use crate::simd::SIMD_WIDTH;
use crate::simd::Simd;
use crate::simd::simd_int;
use std::fmt::Debug;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::simd::cmp::SimdOrd;
use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdInt;
use std::simd::num::SimdUint;

#[derive(Clone, Copy)]
pub struct LogicArray64 {
    v: u64,
    x: u64,
    width: u8,
}

#[allow(non_upper_case_globals)]
impl LogicArray64 {
    const SIMD_0: Simd = Simd::splat(0 as simd_int);
    const SIMD_0b01: Simd = Simd::splat(0b01 as simd_int);
    const SIMD_0b10: Simd = Simd::splat(0b10 as simd_int);
}

impl Debug for LogicArray64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogicVecU64")
            .field("v", &format!("{:#08b}", self.v))
            .field("x", &format!("{:#08b}", self.x))
            .field("width", &self.width)
            .finish()
    }
}

impl LogicSlice for LogicArray64 {
    const MAX_WDITH: usize = 64;

    fn width(&self) -> usize {
        self.width as usize
    }

    fn nand(a: &Self, b: &Self) -> Self {
        Self::not(&Self::and(a, b))
    }

    fn not(a: &Self) -> Self {
        LogicArray64 {
            v: !a.v,
            x: a.x,
            width: a.width,
        }
    }

    fn and(a: &Self, b: &Self) -> Self {
        LogicArray64 {
            v: a.v & b.v,
            x: (a.x & b.x) | (a.v & b.x) | (b.v & a.x),
            width: u8::min(a.width, b.width),
        }
    }

    fn or(a: &Self, b: &Self) -> Self {
        LogicArray64 {
            v: a.v | b.v,
            x: (a.x & b.x) | (!a.v & b.x) | (!b.v & a.x),
            width: u8::min(a.width, b.width),
        }
    }

    fn from_logics(logics: &[Logic]) -> Self {
        if logics.len() > Self::MAX_WDITH {
            panic!("illegal argument");
        }

        let num_simd = __div_ceil(logics.len(), SIMD_WIDTH);

        // packed-v-mask: [0b00101...]
        let mut v = 0;
        // packed-x-mask: [0b00101...]
        let mut x = 0;

        for i in 0..num_simd {
            let slice = &logics[i * SIMD_WIDTH..usize::min((i + 1) * SIMD_WIDTH, logics.len())];

            // [0bvx, 0bvx, ...]
            let simd_logics: Simd = Simd::load_or(
                Logic::as_logic_int_slice(slice),
                Simd::<logic_int>::splat(Logic::X as logic_int),
            )
            .cast();

            let simd_0 = Self::SIMD_0;
            // v-mask: [0b00...|0bFF..., 0b00...|0bFF...]
            let mask_v = Simd::simd_gt(Simd::bitand(simd_logics, Self::SIMD_0b01), simd_0);
            // x-mask: [0b00...|0bFF..., 0b00...|0bFF...]
            let mask_x = Simd::simd_gt(Simd::bitand(simd_logics, Self::SIMD_0b10), simd_0);

            v |= mask_v.to_bitmask() << (i * SIMD_WIDTH);
            x |= mask_x.to_bitmask() << (i * SIMD_WIDTH);
        }

        LogicArray64 {
            v,
            x,
            width: logics.len() as u8,
        }
    }

    fn to_logics(array: &LogicArray64, out: &mut [Logic]) {
        if out.len() != array.width() {
            panic!("illegal argument");
        }

        let num_simd = __div_ceil(array.width(), SIMD_WIDTH);

        for i in 0..num_simd {
            // v-mask: [0b00...|0bFF..., 0b00...|0bFF...]
            let mask_v: Mask = Mask::from_bitmask(array.v >> (i * SIMD_WIDTH));
            // x-mask: [0b00...|0bFF..., 0b00...|0bFF...]
            let mask_x: Mask = Mask::from_bitmask(array.x >> (i * SIMD_WIDTH));

            let simd_logics = Simd::simd_min(
                Simd::bitor(
                    Simd::bitand(mask_v.to_int().cast(), Self::SIMD_0b01),
                    Simd::bitand(mask_x.to_int().cast(), Self::SIMD_0b10),
                ),
                Simd::splat(3 as simd_int),
            );

            let out_slice: &mut [logic_int] = Logic::as_logic_int_slice_mut(
                &mut out[i * SIMD_WIDTH..usize::min((i + 1) * SIMD_WIDTH, array.width())],
            );

            simd_logics
                .cast::<logic_int>()
                .store_select(out_slice, Mask::splat(true));
        }
    }
}
