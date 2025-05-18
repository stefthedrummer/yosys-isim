#![allow(non_camel_case_types)]

use std::fmt::Debug;
use std::mem::transmute;
use yosys_isim_macros::define_enum;

#[cfg(feature = "logic_int_u8")]
pub type logic_int = u8;
#[cfg(feature = "logic_int_u8")]
define_enum![enum Logic repr(u8) derive(Copy, Clone, Debug, Eq, PartialEq) {
    _0,
    _1,
    X,
}];

#[cfg(feature = "logic_int_u32")]
pub type logic_int = u32;
#[cfg(feature = "logic_int_u32")]
define_enum![enum Logic repr(u32) derive(Copy, Clone, Debug, Eq, PartialEq) {
    _0,
    _1,
    X,
}];

impl Logic {
    pub fn as_logic_int_slice(logics: &[Logic]) -> &[logic_int] {
        unsafe { transmute(logics) }
    }

    pub fn as_logic_int_slice_mut(logics: &mut [Logic]) -> &mut [logic_int] {
        unsafe { transmute(logics) }
    }
}
