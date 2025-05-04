use crate::{common::Vec4, model::HWire};

pub struct SimPort<const L: usize> {
    pub(super) h_wires: Vec4<HWire>,
}
