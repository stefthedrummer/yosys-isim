#![allow(non_upper_case_globals)]
use crate::define_enum;
use crate::model::Logic;
use crate::sim::BINARY_OP_FNS;
use crate::sim::UNARY_FNS;

define_enum![enum UnaryMapOp repr(u8) {
    NOT,
    BUF,
}];

impl UnaryMapOp {
    pub fn eval_bool(op: UnaryMapOp, a: bool) -> bool {
        match op {
            UnaryMapOp::NOT => !a,
            UnaryMapOp::BUF => a,
        }
    }
}

define_enum![enum BinaryMapOp repr(u8) {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
    AND_NOT,
    OR_NOT,
}];

impl BinaryMapOp {
    pub fn eval_bool(op: BinaryMapOp, a: bool, b: bool) -> bool {
        match op {
            BinaryMapOp::AND => a & b,
            BinaryMapOp::OR => a | b,
            BinaryMapOp::XOR => a ^ b,
            BinaryMapOp::NAND => !(a & b),
            BinaryMapOp::NOR => !(a | b),
            BinaryMapOp::XNOR => !(a ^ b),
            BinaryMapOp::AND_NOT => a & !b,
            BinaryMapOp::OR_NOT => a | !b,
        }
    }
}

define_enum![enum TernaryMapOp repr(u8) {
    AND_OR_INV,
    OR_AND_INV,
}];

impl TernaryMapOp {
    pub fn eval_logic(op: TernaryMapOp, a: Logic, b: Logic, c: Logic) -> Logic {
        let not = &UNARY_FNS[UnaryMapOp::NOT];
        let and = &BINARY_OP_FNS[BinaryMapOp::AND];
        let or = &BINARY_OP_FNS[BinaryMapOp::OR];

        match op {
            TernaryMapOp::AND_OR_INV => not[or[(and[(a, b)], c)]],
            TernaryMapOp::OR_AND_INV => not[and[(or[(a, b)], c)]],
        }
    }
}
