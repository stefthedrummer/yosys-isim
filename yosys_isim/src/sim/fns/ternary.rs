use crate::model::Logic;
use crate::model::Logic_Variants;
use crate::ops::TernaryMapOp;
use crate::ops::TernaryMapOp_Len;
use crate::ops::TernaryMapOp_Variants;
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct TernaryMapFn {
    table: [[[Logic; 3]; 3]; 3],
}

impl Index<TernaryMapOp> for [TernaryMapFn; TernaryMapOp_Len] {
    type Output = TernaryMapFn;

    fn index(&self, index: TernaryMapOp) -> &TernaryMapFn {
        &self[index as usize]
    }
}

impl Index<(Logic, Logic, Logic)> for TernaryMapFn {
    type Output = Logic;

    fn index(&self, index: (Logic, Logic, Logic)) -> &Self::Output {
        &self.table[index.0 as usize][index.1 as usize][index.2 as usize]
    }
}

impl TernaryMapFn {
    pub(super) fn compile_all() -> [TernaryMapFn; TernaryMapOp_Len] {
        let mut fs: [TernaryMapFn; TernaryMapOp_Len] = [TernaryMapFn {
            table: [[[Logic::X; 3]; 3]; 3],
        }; TernaryMapOp_Len];

        for (index, op) in TernaryMapOp_Variants.iter().enumerate() {
            fs[index] = TernaryMapFn::compile(*op);
        }

        fs
    }

    fn compile(op: TernaryMapOp) -> TernaryMapFn {
        let mut f = TernaryMapFn {
            table: [[[Logic::X; 3]; 3]; 3],
        };

        for a in Logic_Variants.into_iter() {
            for b in Logic_Variants.into_iter() {
                for c in Logic_Variants.into_iter() {
                    f.table[a as usize][b as usize][c as usize] =
                        TernaryMapOp::eval_logic(op, a, b, c);
                }
            }
        }

        f
    }
}
