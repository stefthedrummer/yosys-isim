use crate::common::Vec4;
use crate::model::AddCell;
use crate::model::BinaryMapOpCell;
use crate::model::Cell;
use crate::model::CellInPort;
use crate::model::CellOutPort;
use crate::model::DFlipFlopCell;
use crate::model::Edge;
use crate::model::HWire;
use crate::model::HWireOrLogic;
use crate::model::Logic;
use crate::model::MuxCell;
use crate::model::TernaryMapOpCell;
use crate::model::UnaryMapOpCell;
use crate::ops::BinaryMapOp;
use crate::ops::UnaryMapOp;
use crate::sim::SimState;
use crate::sim::StateRef;
use enum_dispatch::enum_dispatch;
use smallvec::smallvec;

#[enum_dispatch]
pub trait CellSimModel {
    fn name(&self) -> &str;
    fn in_ports(&self) -> Vec4<&CellInPort>;
    fn out_ports(&self) -> Vec4<&CellOutPort>;
    fn simulate(&self, sim: &mut SimState);
}

pub struct CellWires {}
impl CellWires {
    pub fn get_in_port_h_wires<'a>(cell: &'a Cell) -> impl Iterator<Item = HWire> + 'a {
        cell.in_ports()
            .into_iter()
            .flat_map(|port| port.wires.iter())
            .filter_map(|wire| match wire {
                HWireOrLogic::HWire(h_wire) => Some(h_wire),
                HWireOrLogic::Logic(_) => None,
            })
            .cloned()
    }

    pub fn get_out_port_h_wires<'a>(cell: &'a Cell) -> impl Iterator<Item = HWire> + 'a {
        cell.out_ports()
            .into_iter()
            .flat_map(|port| port.wires.iter())
            .cloned()
    }

    pub fn get_all_h_wires<'a>(cell: &'a Cell) -> impl Iterator<Item = HWire> + 'a {
        Self::get_in_port_h_wires(&cell).chain(Self::get_out_port_h_wires(&cell))
    }
}

impl CellSimModel for UnaryMapOpCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&CellInPort> {
        Vec4::from_slice(&[&self.port_a])
    }
    fn out_ports(&self) -> Vec4<&CellOutPort> {
        Vec4::from_slice(&[&self.port_y])
    }
    fn simulate(&self, sim: &mut SimState) {
        let mut a: Vec4<Logic> = smallvec![Logic::X; self.port_a.wires.len()];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.wires.len()];

        sim.get_wires_or_logic(StateRef::Cur, &self.port_a.wires, &mut a);

        let not = &sim.ops.unary[UnaryMapOp::NOT];

        for i in 0..a.len() {
            y[i] = not[a[i]];
        }

        sim.set_wires(StateRef::Cur, &self.port_y.wires, &y);
    }
}

impl CellSimModel for BinaryMapOpCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&CellInPort> {
        Vec4::from_slice(&[&self.port_a, &self.port_b])
    }
    fn out_ports(&self) -> Vec4<&CellOutPort> {
        Vec4::from_slice(&[&self.port_y])
    }
    fn simulate(&self, sim: &mut SimState) {
        let mut a: Vec4<Logic> = smallvec![Logic::X; self.port_a.wires.len()];
        let mut b: Vec4<Logic> = smallvec![Logic::X; self.port_b.wires.len()];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.wires.len()];

        sim.get_wires_or_logic(StateRef::Cur, &self.port_a.wires, &mut a);
        sim.get_wires_or_logic(StateRef::Cur, &self.port_b.wires, &mut b);

        let op = &sim.ops.binary[self.op];
        for i in 0..a.len() {
            y[i] = op[(a[i], b[i])];
        }

        sim.set_wires(StateRef::Cur, &self.port_y.wires, &y);
    }
}

impl CellSimModel for DFlipFlopCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&CellInPort> {
        Vec4::from_slice(&[&self.port_clk, &self.port_d])
    }
    fn out_ports(&self) -> Vec4<&CellOutPort> {
        Vec4::from_slice(&[&self.port_q])
    }
    fn simulate(&self, sim: &mut SimState) {
        assert_eq!(self.port_clk.wires.len(), 1);

        let mut clk: [Edge; 1] = [Edge::X; 1];
        sim.get_edges(&self.port_clk.wires, &mut clk);

        match clk[0] {
            Edge::NONE => (),
            Edge::X => (),
            Edge::POSITIVE | Edge::NEGATIVE => {
                if clk[0] == self.polarity {
                    let mut d: Vec4<Logic> = smallvec![Logic::X; self.port_d.wires.len()];
                    sim.get_wires_or_logic(StateRef::Cur, &self.port_d.wires, &mut d);
                    sim.set_wires_deferred(&self.port_q.wires, &mut d);
                }
            }
        }
    }
}

impl CellSimModel for AddCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&CellInPort> {
        Vec4::from_slice(&[&self.port_a, &self.port_b])
    }
    fn out_ports(&self) -> Vec4<&CellOutPort> {
        Vec4::from_slice(&[&self.port_y])
    }

    fn simulate(&self, sim: &mut SimState) {
        let mut a: Vec4<Logic> = smallvec![Logic::X; self.port_a.wires.len()];
        let mut b: Vec4<Logic> = smallvec![Logic::X; self.port_b.wires.len()];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.wires.len()];

        sim.get_wires_or_logic(StateRef::Cur, &self.port_a.wires, &mut a);
        sim.get_wires_or_logic(StateRef::Cur, &self.port_b.wires, &mut b);

        let xor = &sim.ops.binary[BinaryMapOp::XOR];
        let and = &sim.ops.binary[BinaryMapOp::AND];
        let or = &sim.ops.binary[BinaryMapOp::OR];

        let mut c = Logic::_0;

        for i in 0..y.len() {
            let a_xor_b = xor[(a[i], b[i])];
            y[i] = xor[(a_xor_b, c)];
            c = or[(and[(a_xor_b, c)], and[(a[i], b[i])])];
        }

        sim.set_wires(StateRef::Cur, &self.port_y.wires, &y);
    }
}

impl CellSimModel for TernaryMapOpCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&CellInPort> {
        Vec4::from_slice(&[&self.port_a, &self.port_b, &self.port_c])
    }
    fn out_ports(&self) -> Vec4<&CellOutPort> {
        Vec4::from_slice(&[&self.port_y])
    }
    fn simulate(&self, sim: &mut SimState) {
        let mut a: Vec4<Logic> = smallvec![Logic::X; self.port_a.wires.len()];
        let mut b: Vec4<Logic> = smallvec![Logic::X; self.port_b.wires.len()];
        let mut c: Vec4<Logic> = smallvec![Logic::X; self.port_c.wires.len()];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.wires.len()];

        sim.get_wires_or_logic(StateRef::Cur, &self.port_a.wires, &mut a);
        sim.get_wires_or_logic(StateRef::Cur, &self.port_b.wires, &mut b);
        sim.get_wires_or_logic(StateRef::Cur, &self.port_c.wires, &mut c);

        let op = &sim.ops.ternary[self.op];

        for i in 0..y.len() {
            y[i] = op[(a[i], b[i], c[i])];
        }

        sim.set_wires(StateRef::Cur, &self.port_y.wires, &y);
    }
}

impl CellSimModel for MuxCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&CellInPort> {
        Vec4::from_slice(&[&self.port_a, &self.port_b, &self.port_s.as_ref_0()])
    }
    fn out_ports(&self) -> Vec4<&CellOutPort> {
        Vec4::from_slice(&[&self.port_y])
    }
    fn simulate(&self, sim: &mut SimState) {
        let s: Vec4<Logic> = smallvec![Logic::X; 1];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.wires.len()];

        sim.get_wires_or_logic(
            StateRef::Cur,
            match s[0] {
                Logic::_0 => &self.port_a.wires,
                Logic::_1 => &self.port_b.wires,
                Logic::X => todo!("not implemented"),
            },
            &mut y,
        );

        sim.set_wires(StateRef::Cur, &self.port_y.wires, &y);
    }
}
