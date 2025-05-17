#![allow(non_upper_case_globals)]

use crate::common::Vec2;
use crate::define_enum;
use core::str;
use smallvec::smallvec;

define_enum![enum Logic repr(u8) derive(Copy, Clone, Debug) {
    _0, _1 , X,
}];

impl Logic {
    pub fn to_bits<const L: usize>(integer: isize) -> [Logic; L] {
        let mut bits: [Logic; L] = [Logic::X; L];
        for i in 0..L {
            bits[i] = ((integer >> i) & 0b1).into();
        }
        bits
    }

    const fn from_bool(value: bool) -> Logic {
        match value {
            false => Self::_0,
            true => Self::_1,
        }
    }

    pub fn from_str(value: &str) -> Option<Logic> {
        match value {
            "0" => Some(Logic::_0),
            "1" => Some(Logic::_1),
            "X" => Some(Logic::X),
            _ => None,
        }
    }

    pub fn to_bool_set(logic: Logic) -> Vec2<bool> {
        match logic {
            Logic::_0 => smallvec![false],
            Logic::_1 => smallvec![true],
            Logic::X => smallvec![false, true],
        }
    }

    pub fn from_bool_set(logic: &[bool]) -> Logic {
        if logic.len() == 0 {
            panic!("illega argument");
        }
        let first = logic[0];
        match logic.iter().all(|it| *it == first) {
            true => Self::from_bool(first),
            false => Logic::X,
        }
    }

    pub fn is_physical(&self) -> bool {
        *self as u8 <= Logic::_1 as u8
    }

    pub fn is_eq_logical(a: Logic, b: Logic) -> bool {
        a as u8 == b as u8
    }

    pub fn is_eq_physical(a: Logic, b: Logic) -> bool {
        a.is_physical() & b.is_physical() & (a as u8 == b as u8)
    }

    pub fn is_slice_eq(a: &[Logic], b: &[Logic], equality: impl Fn(Logic, Logic) -> bool) -> bool {
        let len_a = a.len();
        let len_b = b.len();
        if len_a != len_b {
            return false;
        }
        for i in 0..len_a {
            if !equality(a[i], b[i]) {
                return false;
            }
        }
        return true;
    }
}

impl From<bool> for Logic {
    fn from(value: bool) -> Self {
        Logic::from_bool(value)
    }
}

macro_rules! from_impl {
    ($type:ty) => {
        impl From<$type> for Logic {
            fn from(value: $type) -> Self {
                match value {
                    0 => Self::_0,
                    1 => Self::_1,
                    _ => panic!("cannot convert [{}] to State", value),
                }
            }
        }
    };
}
from_impl![u8];
from_impl![i8];
from_impl![u32];
from_impl![u64];
from_impl![i32];
from_impl![i64];
from_impl![usize];
from_impl![isize];
