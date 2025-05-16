pub mod binary;
pub mod ternary;
pub mod unary;

use crate::ops::BinaryOp_Len;
use crate::ops::TernaryOp_Len;
use crate::ops::UnaryOp_Len;
pub use binary::*;
use lazy_static::lazy_static;
pub use ternary::*;
pub use unary::*;

lazy_static! {
    pub static ref UNARY_FNS: [UnaryOpFn; UnaryOp_Len] = UnaryOpFn::compile_all();
    pub static ref BINARY_OP_FNS: [BinaryOpFn; BinaryOp_Len] = BinaryOpFn::compile_all();
    pub static ref TERNARY_OP_FNS: [TernaryFn; TernaryOp_Len] = TernaryFn::compile_all();
    pub static ref OP_FNS: OpFns = OpFns {
        unary: *UNARY_FNS,
        binary: *BINARY_OP_FNS,
        ternary: *TERNARY_OP_FNS,
    };
}

#[derive(Clone)]
pub struct OpFns {
    pub unary: [UnaryOpFn; UnaryOp_Len],
    pub binary: [BinaryOpFn; BinaryOp_Len],
    pub ternary: [TernaryFn; TernaryOp_Len],
}
