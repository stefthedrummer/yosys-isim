use crate::common::Vec4;
use crate::model::Logic;
use crate::model::Logic_Variants;
use crate::ops::BinaryMapOp;
use crate::ops::BinaryMapOp_Len;
use crate::ops::BinaryMapOp_Variants;
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct BinaryMapOpFn {
    table: [[Logic; 3]; 3],
}

impl Index<BinaryMapOp> for [BinaryMapOpFn; BinaryMapOp_Len] {
    type Output = BinaryMapOpFn;

    fn index(&self, index: BinaryMapOp) -> &BinaryMapOpFn {
        &self[index as usize]
    }
}

impl Index<(Logic, Logic)> for BinaryMapOpFn {
    type Output = Logic;

    fn index(&self, index: (Logic, Logic)) -> &Logic {
        &self.table[index.0 as usize][index.1 as usize]
    }
}

impl BinaryMapOpFn {
    pub(super) fn compile_all() -> [BinaryMapOpFn; BinaryMapOp_Len] {
        let mut fs: [BinaryMapOpFn; BinaryMapOp_Len] = [BinaryMapOpFn {
            table: [[Logic::X; 3]; 3],
        }; BinaryMapOp_Len];

        for (index, op) in BinaryMapOp_Variants.iter().enumerate() {
            fs[index] = BinaryMapOpFn::compile(*op);
        }

        fs
    }

    fn compile(op: BinaryMapOp) -> BinaryMapOpFn {
        let mut f = BinaryMapOpFn {
            table: [[Logic::X; 3]; 3],
        };
        for a in Logic_Variants.into_iter() {
            for b in Logic_Variants.into_iter() {
                f.table[a as usize][b as usize] = Self::eval_logic(op, a, b);
            }
        }
        f
    }

    fn eval_logic(op: BinaryMapOp, a: Logic, b: Logic) -> Logic {
        let mut out_bool_set: Vec4<bool> = Vec4::new();

        for a_bool in Logic::to_bool_set(a) {
            for b_bool in Logic::to_bool_set(b) {
                out_bool_set.push(BinaryMapOp::eval_bool(op, a_bool, b_bool));
            }
        }

        Logic::from_bool_set(&out_bool_set)
    }
}
