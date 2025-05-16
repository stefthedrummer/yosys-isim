use crate::common::FindByName;
use crate::common::HasName;
use crate::common::SimError;
use crate::common::Vec4;
use crate::model::Cell;
use crate::model::Dir;
use crate::model::HWire;
use crate::model::In;
use crate::model::Out;
use crate::model::Port;

pub type ModuleInPort<const L: usize = 0> = Port<In, HWire, L>;
pub type ModuleOutPort<const L: usize = 0> = Port<Out, HWire, L>;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub cells: Vec<Cell>,
    pub in_ports: Vec4<ModuleInPort>,
    pub out_ports: Vec4<ModuleOutPort>,
}

impl Module {
    pub fn get_in_port<const L: usize>(&self, name: &str) -> Result<ModuleInPort<L>, SimError> {
        Ok(self
            .get_port_dynamic(&self.in_ports, name, L)?
            .clone()
            .into_width::<L>())
    }

    pub fn get_out_port<const L: usize>(&self, name: &str) -> Result<ModuleOutPort<L>, SimError> {
        Ok(self
            .get_port_dynamic(&self.out_ports, name, L)?
            .clone()
            .into_width::<L>())
    }

    pub fn get_port_dynamic<'a, D: Dir, W>(
        &self,
        ports: &'a [Port<D, W>],
        name: &str,
        width: usize,
    ) -> Result<&'a Port<D, W>, SimError> {
        let port = ports.iter().find_by_name(name)?;

        if port.wires.len() != width {
            Err(SimError::SimError {
                msg: format!(
                    "wrong port width [{}] on [{}],  actual is [{}]",
                    width,
                    name,
                    port.wires.len()
                ),
            })?;
        }

        Ok(&port)
    }
}

impl HasName for Module {
    const LABEL: &'static str = "module";
    fn name(&self) -> &str {
        &self.name
    }
}
