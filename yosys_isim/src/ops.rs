#![allow(non_upper_case_globals)]
use crate::{BINARY_OP_FNS, Logic, UNARY_FNS, make_enum};

make_enum![enum UnaryOp repr(u8) {
    NOT,
    BUF,
}];

impl UnaryOp {
    pub fn eval_bool(op: UnaryOp, a: bool) -> bool {
        match op {
            UnaryOp::NOT => !a,
            UnaryOp::BUF => a,
        }
    }
}

make_enum![enum BinaryOp repr(u8) {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
    AND_NOT,
    OR_NOT,
}];

impl BinaryOp {
    pub fn eval_bool(op: BinaryOp, a: bool, b: bool) -> bool {
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

make_enum![enum TernaryOp repr(u8) {
    AND_OR_INV,
    OR_AND_INV,
}];

impl TernaryOp {
    pub fn eval_logic(op: TernaryOp, a: Logic, b: Logic, c: Logic) -> Logic {
        let not = &UNARY_FNS[UnaryOp::NOT];
        let and = &BINARY_OP_FNS[BinaryOp::AND];
        let or = &BINARY_OP_FNS[BinaryOp::OR];

        match op {
            TernaryOp::AND_OR_INV => not[or[(and[(a, b)], c)]],
            TernaryOp::OR_AND_INV => not[and[(or[(a, b)], c)]],
        }
    }
}
