use crate::{
    TernaryOp, TernaryOp_Len, TernaryOp_Variants, Vec4,
    model::{BinaryOp, BinaryOp_Len, BinaryOp_Variants},
};
use lazy_static::lazy_static;
use std::ops::Index;

use super::{Logic, Logic_Variants};

lazy_static! {
    pub static ref NOT_FN: NotOpFn = NotOpFn::compile();
    pub static ref BINARY_OP_FNS: [BinaryOpFn; BinaryOp_Len] = BinaryOpFn::compile_all();
    pub static ref TERNARY_OP_FNS: [TernaryFn; TernaryOp_Len] = TernaryFn::compile_all();
    pub static ref OP_FNS: OpFns = OpFns {
        not: *NOT_FN,
        binary: *BINARY_OP_FNS,
        ternary: *TERNARY_OP_FNS,
    };
}

#[derive(Clone)]
pub struct OpFns {
    pub not: NotOpFn,
    pub binary: [BinaryOpFn; BinaryOp_Len],
    pub ternary: [TernaryFn; TernaryOp_Len],
}

#[derive(Copy, Clone)]
pub struct NotOpFn {
    table: [Logic; 3],
}

impl Index<Logic> for NotOpFn {
    type Output = Logic;

    fn index(&self, index: Logic) -> &Logic {
        &self.table[index as usize]
    }
}

impl NotOpFn {
    fn compile() -> NotOpFn {
        let mut f = NotOpFn {
            table: [Logic::X; 3],
        };
        f.table[Logic::_0 as usize] = Logic::_1;
        f.table[Logic::_1 as usize] = Logic::_0;
        f.table[Logic::X as usize] = Logic::X;
        f
    }
}

#[derive(Copy, Clone)]
pub struct BinaryOpFn {
    table: [[Logic; 3]; 3],
}

impl Index<BinaryOp> for [BinaryOpFn; BinaryOp_Len] {
    type Output = BinaryOpFn;

    fn index(&self, index: BinaryOp) -> &BinaryOpFn {
        &self[index as usize]
    }
}

impl Index<(Logic, Logic)> for BinaryOpFn {
    type Output = Logic;

    fn index(&self, index: (Logic, Logic)) -> &Logic {
        &self.table[index.0 as usize][index.1 as usize]
    }
}

impl BinaryOpFn {
    fn compile_all() -> [BinaryOpFn; BinaryOp_Len] {
        let mut fs: [BinaryOpFn; BinaryOp_Len] = [BinaryOpFn {
            table: [[Logic::X; 3]; 3],
        }; BinaryOp_Len];

        for (index, op) in BinaryOp_Variants.iter().enumerate() {
            fs[index] = BinaryOpFn::compile(*op);
        }

        fs
    }

    fn compile(op: BinaryOp) -> BinaryOpFn {
        let mut f = BinaryOpFn {
            table: [[Logic::X; 3]; 3],
        };
        for a in Logic_Variants.into_iter() {
            for b in Logic_Variants.into_iter() {
                f.table[a as usize][b as usize] = Self::eval_logic(op, a, b);
            }
        }
        f
    }

    fn eval_logic(op: BinaryOp, a: Logic, b: Logic) -> Logic {
        let mut out_bool_set: Vec4<bool> = Vec4::new();

        for a_bool in Logic::to_bool_set(a) {
            for b_bool in Logic::to_bool_set(b) {
                out_bool_set.push(Self::eval_bool(op, a_bool, b_bool));
            }
        }

        Logic::from_bool_set(&out_bool_set)
    }

    fn eval_bool(op: BinaryOp, a: bool, b: bool) -> bool {
        match op {
            BinaryOp::AND => a & b,
            BinaryOp::OR => a | b,
            BinaryOp::XOR => a ^ b,
            BinaryOp::NAND => !(a & b),
            BinaryOp::NOR => !(a | b),
            BinaryOp::XNOR => !(a ^ b),
            BinaryOp::AND_NOT => a & !b,
            BinaryOp::OR_NOT => a | !b,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TernaryFn {
    table: [[[Logic; 3]; 3]; 3],
}

impl Index<TernaryOp> for [TernaryFn; TernaryOp_Len] {
    type Output = TernaryFn;

    fn index(&self, index: TernaryOp) -> &TernaryFn {
        &self[index as usize]
    }
}

impl Index<(Logic, Logic, Logic)> for TernaryFn {
    type Output = Logic;

    fn index(&self, index: (Logic, Logic, Logic)) -> &Self::Output {
        &self.table[index.0 as usize][index.1 as usize][index.2 as usize]
    }
}

impl TernaryFn {
    fn compile_all() -> [TernaryFn; TernaryOp_Len] {
        let mut fs: [TernaryFn; TernaryOp_Len] = [TernaryFn {
            table: [[[Logic::X; 3]; 3]; 3],
        }; TernaryOp_Len];

        for (index, op) in TernaryOp_Variants.iter().enumerate() {
            fs[index] = TernaryFn::compile(*op);
        }

        fs
    }

    fn compile(op: TernaryOp) -> TernaryFn {
        let mut f = TernaryFn {
            table: [[[Logic::X; 3]; 3]; 3],
        };

        for a in Logic_Variants.into_iter() {
            for b in Logic_Variants.into_iter() {
                for c in Logic_Variants.into_iter() {
                    f.table[a as usize][b as usize][c as usize] = Self::eval_logic(op, a, b, c);
                }
            }
        }

        f
    }

    fn eval_logic(op: TernaryOp, a: Logic, b: Logic, c: Logic) -> Logic {
        let not = &NOT_FN;
        let and = &BINARY_OP_FNS[BinaryOp::AND];
        let or = &BINARY_OP_FNS[BinaryOp::OR];

        match op {
            TernaryOp::AND_OR_INV => not[or[(and[(a, b)], c)]],
            TernaryOp::OR_AND_INV => not[and[(or[(a, b)], c)]],
        }
    }
}
