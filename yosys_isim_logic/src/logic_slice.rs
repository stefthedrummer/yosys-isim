use crate::BinaryOp;
use crate::Logic;

pub trait LogicSlice: Sized {
    const MAX_WDITH: usize;

    fn width(&self) -> usize;

    fn not(a: &Self) -> Self;
    fn and(a: &Self, b: &Self) -> Self;
    fn or(a: &Self, b: &Self) -> Self;

    fn nand(a: &Self, b: &Self) -> Self {
        Self::not(&Self::and(a, b))
    }

    fn nor(a: &Self, b: &Self) -> Self {
        Self::not(&Self::or(a, b))
    }

    fn from_logics(logics: &[Logic]) -> Self;
    fn to_logics(a: &Self, logics: &mut [Logic]);

    fn from_logic(logic: Logic) -> Self {
        Self::from_logics(&[logic])
    }

    fn to_logic(a: &Self) -> Logic {
        Self::to_logics_fixed::<1>(a)[0]
    }

    fn to_logics_fixed<const L: usize>(a: &Self) -> [Logic; L] {
        let mut logics = [Logic::X; L];
        Self::to_logics(a, &mut logics);
        logics
    }

    fn binary_op(op: BinaryOp) -> fn(&Self, &Self) -> Self {
        match op {
            BinaryOp::AND => Self::and,
            BinaryOp::OR => Self::or,
            BinaryOp::XOR => todo!(),
            BinaryOp::NAND => Self::nand,
            BinaryOp::NOR => Self::nor,
            BinaryOp::XNOR => todo!(),
            BinaryOp::AND_NOT => todo!(),
            BinaryOp::OR_NOT => todo!(),
        }
    }
}
