use crate::common::HasName;
use crate::common::Vec4;
use crate::model::HWire;

#[derive(Debug, Clone)]
pub struct Port {
    pub name: String,
    pub h_wires: Vec4<HWire>,
}

impl HasName for Port {
    fn name(&self) -> &str {
        &self.name
    }
}
