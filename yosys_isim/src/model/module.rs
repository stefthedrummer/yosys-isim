use crate::common::HasName;
use crate::common::Vec4;
use crate::model::Cell;
use crate::model::Port;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub cells: Vec<Cell>,
    pub in_ports: Vec4<Port>,
    pub out_ports: Vec4<Port>,
}

impl HasName for Module {
    fn name(&self) -> &str {
        &self.name
    }
}
