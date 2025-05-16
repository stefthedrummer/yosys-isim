use crate::{BinaryOp, BinaryOp_Len, BinaryOp_Variants, Logic, Logic_Variants, Vec4};

use std::ops::Index;

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
    pub(super) fn compile_all() -> [BinaryOpFn; BinaryOp_Len] {
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
                out_bool_set.push(BinaryOp::eval_bool(op, a_bool, b_bool));
            }
        }

        Logic::from_bool_set(&out_bool_set)
    }
}
