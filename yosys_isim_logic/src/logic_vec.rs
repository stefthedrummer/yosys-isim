use crate::LogicSlice;

pub struct LogicVec<T: LogicSlice> {
    vec: Vec<T>,
}

impl<T: LogicSlice> LogicSlice for LogicVec<T> {
    const MAX_WDITH: usize = 1024;

    fn width(&self) -> usize {
        todo!()
    }

    fn not(a: &Self) -> Self {
        todo!()
    }

    fn and(a: &Self, b: &Self) -> Self {
        todo!()
    }

    fn or(a: &Self, b: &Self) -> Self {
        todo!()
    }

    fn from_logics(logics: &[crate::Logic]) -> Self {
        todo!()
    }

    fn to_logics(a: &Self, logics: &mut [crate::Logic]) {
        todo!()
    }
}
