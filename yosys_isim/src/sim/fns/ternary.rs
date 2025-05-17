use crate::model::Logic;
use crate::model::Logic_Variants;
use crate::ops::TernaryOp;
use crate::ops::TernaryOp_Len;
use crate::ops::TernaryOp_Variants;
use std::ops::Index;

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
    pub(super) fn compile_all() -> [TernaryFn; TernaryOp_Len] {
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
                    f.table[a as usize][b as usize][c as usize] =
                        TernaryOp::eval_logic(op, a, b, c);
                }
            }
        }

        f
    }
}
