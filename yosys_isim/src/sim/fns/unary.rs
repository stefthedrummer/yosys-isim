use crate::{Logic, Logic_Variants, UnaryOp, UnaryOp_Len, UnaryOp_Variants, Vec4};

use std::ops::Index;

#[derive(Copy, Clone)]
pub struct UnaryOpFn {
    table: [Logic; 3],
}

impl Index<UnaryOp> for [UnaryOpFn; UnaryOp_Len] {
    type Output = UnaryOpFn;

    fn index(&self, index: UnaryOp) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<Logic> for UnaryOpFn {
    type Output = Logic;

    fn index(&self, index: Logic) -> &Logic {
        &self.table[index as usize]
    }
}

impl UnaryOpFn {
    pub(super) fn compile_all() -> [UnaryOpFn; UnaryOp_Len] {
        let mut fs: [UnaryOpFn; UnaryOp_Len] = [UnaryOpFn {
            table: [Logic::X; 3],
        }; UnaryOp_Len];

        for (index, op) in UnaryOp_Variants.iter().enumerate() {
            fs[index] = UnaryOpFn::compile(*op);
        }

        fs
    }

    fn compile(op: UnaryOp) -> UnaryOpFn {
        let mut f = UnaryOpFn {
            table: [Logic::X; 3],
        };
        for a in Logic_Variants.into_iter() {
            f.table[a as usize] = Self::eval_logic(op, a);
        }
        f
    }

    fn eval_logic(op: UnaryOp, a: Logic) -> Logic {
        let mut out_bool_set: Vec4<bool> = Vec4::new();

        for a_bool in Logic::to_bool_set(a) {
            out_bool_set.push(UnaryOp::eval_bool(op, a_bool));
        }

        Logic::from_bool_set(&out_bool_set)
    }

   
}
