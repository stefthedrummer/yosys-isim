use crate::common::SimError;
use crate::common::Vec4;
use crate::sim::Logic;
use smallvec::smallvec;

pub type HWire = usize;

#[derive(Debug, Clone)]
pub enum HWireOrLogic {
    HWire(HWire),
    Logic(Logic),
}

impl HWireOrLogic {
    #[allow(non_snake_case)]
    pub fn only_HWires(wires: &[HWireOrLogic]) -> Result<Vec4<HWire>, SimError> {
        let mut h_wires: Vec4<HWire> = smallvec![ 0usize; wires.len()    ];
        for i in 0..h_wires.len() {
            h_wires[i] = match wires[i] {
                HWireOrLogic::HWire(h_wire) => h_wire,
                HWireOrLogic::Logic(_) => {
                    return Err(SimError::IllegalState {
                        msg: "wires contained constant Logic values, which is unexpected"
                            .to_string(),
                    });
                }
            }
        }
        Ok(h_wires)
    }
}
