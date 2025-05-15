#![allow(non_upper_case_globals)]

use smallvec::smallvec;

use crate::{Vec2, make_enum};

make_enum![enum Logic repr(u8) {
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

    const fn from(value: bool) -> Self {
        match value {
            false => Self::_0,
            true => Self::_1,
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
            true => Self::from(first),
            false => Logic::X,
        }
    }

    pub fn eq<const W: usize>(a: &[Logic; W], b: &[Logic; W]) -> bool {
        let len_a = a.len();
        let len_b = b.len();
        if len_a != len_b {
            return false;
        }
        for i in 0..len_a {
            if a[i] != b[i] {
                return false;
            }
        }
        return true;
    }
}

impl From<bool> for Logic {
    fn from(value: bool) -> Self {
        Logic::from(value)
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
from_impl![u32];
from_impl![i32];
from_impl![usize];
from_impl![isize];
