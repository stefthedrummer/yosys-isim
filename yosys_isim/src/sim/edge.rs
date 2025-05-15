#![allow(non_upper_case_globals)]

use crate::make_enum;

use super::Logic;

make_enum![enum Edge repr(u8) {
    NONE,
    X,
    POSITIVE,
    NEGATIVE,
}];

impl Edge {
    pub fn of(from: Logic, to: Logic) -> Edge {
        match from {
            Logic::_0 => match to {
                Logic::_0 => Edge::NONE,
                Logic::_1 => Edge::POSITIVE,
                Logic::X => Edge::X,
            },
            Logic::_1 => match to {
                Logic::_0 => Edge::NEGATIVE,
                Logic::_1 => Edge::NONE,
                Logic::X => Edge::X,
            },
            Logic::X => Edge::X,
        }
    }
}
