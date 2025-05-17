use crate::common::Vec4;
use crate::model::Logic;
use crate::model::Logic_Variants;
use crate::ops::UnaryMapOp;
use crate::ops::UnaryMapOp_Len;
use crate::ops::UnaryMapOp_Variants;
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct UnaryMapOpFn {
    table: [Logic; 3],
}

impl Index<UnaryMapOp> for [UnaryMapOpFn; UnaryMapOp_Len] {
    type Output = UnaryMapOpFn;

    fn index(&self, index: UnaryMapOp) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<Logic> for UnaryMapOpFn {
    type Output = Logic;

    fn index(&self, index: Logic) -> &Logic {
        &self.table[index as usize]
    }
}

impl UnaryMapOpFn {
    pub(super) fn compile_all() -> [UnaryMapOpFn; UnaryMapOp_Len] {
        let mut fs: [UnaryMapOpFn; UnaryMapOp_Len] = [UnaryMapOpFn {
            table: [Logic::X; 3],
        }; UnaryMapOp_Len];

        for (index, op) in UnaryMapOp_Variants.iter().enumerate() {
            fs[index] = UnaryMapOpFn::compile(*op);
        }

        fs
    }

    fn compile(op: UnaryMapOp) -> UnaryMapOpFn {
        let mut f = UnaryMapOpFn {
            table: [Logic::X; 3],
        };
        for a in Logic_Variants.into_iter() {
            f.table[a as usize] = Self::eval_logic(op, a);
        }
        f
    }

    fn eval_logic(op: UnaryMapOp, a: Logic) -> Logic {
        let mut out_bool_set: Vec4<bool> = Vec4::new();

        for a_bool in Logic::to_bool_set(a) {
            out_bool_set.push(UnaryMapOp::eval_bool(op, a_bool));
        }

        Logic::from_bool_set(&out_bool_set)
    }
}
