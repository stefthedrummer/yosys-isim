#![allow(unused)]

use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    usize,
};

use crate::model::{BinaryOp, BinaryOp_Len, BinaryOp_Variants};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Edge {
    NONE,
    X,
    POSITIVE,
    NEGATIVE,
}

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

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Logic {
    _0 = 0,
    _1 = 1,
    X = 2,
}

impl Logic {
    const fn eval_bool(binop: BinaryOp, a: bool, b: bool) -> bool {
        match binop {
            BinaryOp::AND => a & b,
            BinaryOp::OR => a | b,
            BinaryOp::XOR => a ^ b,
            BinaryOp::NAND => !(a & b),
            BinaryOp::NOR => !(a | b),
            BinaryOp::XNOR => !(a ^ b),
        }
    }

    const fn eval_logic_bool_bool(binop: BinaryOp, a: Logic, b: Logic) -> Logic {
        Logic::from(Logic::eval_bool(binop, a.into(), b.into()))
    }

    const fn eval_logic_bool_x(binop: BinaryOp, v: Logic) -> Logic {
        let op0: Logic = Logic::eval_logic_bool_bool(binop, v, Logic::_0);
        let op1: Logic = Logic::eval_logic_bool_bool(binop, v, Logic::_1);
        if op0 as u8 == op1 as u8 {
            op0
        } else {
            Logic::X
        }
    }

    pub const fn eval_logic(binop: BinaryOp, a: Logic, b: Logic) -> Logic {
        match (a, b) {
            (Logic::_0, Logic::_0) => Logic::eval_logic_bool_bool(binop, a, b),
            (Logic::_0, Logic::_1) => Logic::eval_logic_bool_bool(binop, a, b),
            (Logic::_1, Logic::_0) => Logic::eval_logic_bool_bool(binop, a, b),
            (Logic::_1, Logic::_1) => Logic::eval_logic_bool_bool(binop, a, b),
            (Logic::_0, Logic::X) => Logic::eval_logic_bool_x(binop, a),
            (Logic::_1, Logic::X) => Logic::eval_logic_bool_x(binop, a),
            (Logic::X, Logic::_0) => Logic::eval_logic_bool_x(binop, b),
            (Logic::X, Logic::_1) => Logic::eval_logic_bool_x(binop, b),
            (Logic::X, Logic::X) => Logic::X,
        }
    }

    const fn from(value: bool) -> Self {
        match value {
            false => Self::_0,
            true => Self::_1,
        }
    }

    const fn into(self) -> bool {
        match self {
            Logic::_0 => false,
            Logic::_1 => true,
            Logic::X => panic!("Logic::X::into!"),
        }
    }

    pub fn eq<const W: usize>(a: &[Logic; W], b: &[Logic; W]) -> bool {
        let len_a = a.len();
        let len_b = b.len();
        if (len_a != len_b) {
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

#[derive(Copy, Clone)]
pub struct BinOpTruthTable {
    matrix: [[Logic; 3]; 3],
}
impl Default for BinOpTruthTable {
    fn default() -> Self {
        Self {
            matrix: [[Logic::X; 3]; 3],
        }
    }
}

impl Index<(Logic, Logic)> for BinOpTruthTable {
    type Output = Logic;

    fn index(&self, index: (Logic, Logic)) -> &Self::Output {
        &self.matrix[index.0 as usize][index.1 as usize]
    }
}

impl IndexMut<(Logic, Logic)> for BinOpTruthTable {
    fn index_mut(&mut self, index: (Logic, Logic)) -> &mut Self::Output {
        &mut self.matrix[index.0 as usize][index.1 as usize]
    }
}

impl BinOpTruthTable {
    pub fn compile() -> [BinOpTruthTable; BinaryOp_Len] {
        let mut tables: [BinOpTruthTable; BinaryOp_Len] =
            [BinOpTruthTable::default(); BinaryOp_Len];

        for (index, binop) in BinaryOp_Variants.iter().enumerate() {
            tables[index] = Self::compile_table(*binop);
        }

        tables
    }

    fn compile_table(binop: BinaryOp) -> BinOpTruthTable {
        let mut table = BinOpTruthTable {
            matrix: [[Logic::X; 3]; 3],
        };
        Self::fill(&mut table, binop, Logic::_0, Logic::_0);
        Self::fill(&mut table, binop, Logic::_0, Logic::_1);
        Self::fill(&mut table, binop, Logic::_1, Logic::_0);
        Self::fill(&mut table, binop, Logic::_1, Logic::_1);
        Self::fill(&mut table, binop, Logic::_0, Logic::X);
        Self::fill(&mut table, binop, Logic::_1, Logic::X);
        Self::fill(&mut table, binop, Logic::X, Logic::_0);
        Self::fill(&mut table, binop, Logic::X, Logic::_1);
        Self::fill(&mut table, binop, Logic::X, Logic::X);
        table
    }

    const fn fill(table: &mut BinOpTruthTable, binop: BinaryOp, a: Logic, b: Logic) {
        table.matrix[a as usize][b as usize] = Logic::eval_logic(binop, a, b);
    }
}
