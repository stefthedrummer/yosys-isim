pub mod binary;
pub mod ternary;
pub mod unary;

use crate::ops::BinaryMapOp_Len;
use crate::ops::TernaryMapOp_Len;
use crate::ops::UnaryMapOp_Len;
pub use binary::*;
use lazy_static::lazy_static;
pub use ternary::*;
pub use unary::*;

lazy_static! {
    pub static ref UNARY_FNS: [UnaryMapOpFn; UnaryMapOp_Len] = UnaryMapOpFn::compile_all();
    pub static ref BINARY_OP_FNS: [BinaryMapOpFn; BinaryMapOp_Len] = BinaryMapOpFn::compile_all();
    pub static ref TERNARY_OP_FNS: [TernaryMapFn; TernaryMapOp_Len] = TernaryMapFn::compile_all();
    pub static ref OP_FNS: OpFns = OpFns {
        unary: *UNARY_FNS,
        binary: *BINARY_OP_FNS,
        ternary: *TERNARY_OP_FNS,
    };
}

#[derive(Clone)]
pub struct OpFns {
    pub unary: [UnaryMapOpFn; UnaryMapOp_Len],
    pub binary: [BinaryMapOpFn; BinaryMapOp_Len],
    pub ternary: [TernaryMapFn; TernaryMapOp_Len],
}
