use crate::{
    common::Vec4,
    model::{AddCell, Cell, DFlipFlopCell},
    sim::{Edge, SimState, WireState},
};
use enum_dispatch::enum_dispatch;
use smallvec::smallvec;

use crate::model::{BinaryOpCell, NotOpCell, Port};

use super::Logic;

#[enum_dispatch]
pub trait SimCell {
    fn name(&self) -> &str;
    fn in_ports(&self) -> Vec4<&Port>;
    fn out_ports(&self) -> Vec4<&Port>;
    fn simulate(&self, sim: &mut SimState);
}

impl SimCell for NotOpCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_a])
    }
    fn out_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_y])
    }
    fn simulate(&self, sim: &mut SimState) {
        let mut a: Vec4<Logic> = smallvec![Logic::X; self.port_a.h_wires.len()];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.h_wires.len()];

        sim.get_wires(WireState::Cur, &self.port_a.h_wires, &mut a);

        for i in 0..a.len() {
            y[i] = match a[i] {
                Logic::_0 => Logic::_1,
                Logic::_1 => Logic::_0,
                Logic::X => Logic::X,
            };
        }

        sim.set_wires(WireState::Cur, &self.port_y.h_wires, &y);
    }
}

impl SimCell for BinaryOpCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_a, &self.port_b])
    }
    fn out_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_y])
    }
    fn simulate(&self, sim: &mut SimState) {
        let mut a: Vec4<Logic> = smallvec![Logic::X; self.port_a.h_wires.len()];
        let mut b: Vec4<Logic> = smallvec![Logic::X; self.port_b.h_wires.len()];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.h_wires.len()];

        sim.get_wires(WireState::Cur, &self.port_a.h_wires, &mut a);
        sim.get_wires(WireState::Cur, &self.port_b.h_wires, &mut b);

        let truth_table = &sim.binop_truthtables[self.op as usize];
        for i in 0..a.len() {
            y[i] = truth_table[(a[i], b[i])];
        }

        sim.set_wires(WireState::Cur, &self.port_y.h_wires, &y);
    }
}

impl SimCell for DFlipFlopCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_clk, &self.port_d])
    }
    fn out_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_q])
    }
    fn simulate(&self, sim: &mut SimState) {
        assert_eq!(self.port_clk.h_wires.len(), 1);

        let mut clk: [Edge; 1] = [Edge::X; 1];
        sim.get_edges(&self.port_clk.h_wires, &mut clk);

        match clk[0] {
            Edge::NONE => (),
            Edge::X => (),
            Edge::POSITIVE | Edge::NEGATIVE => {
                if clk[0] == self.polarity {
                    let mut d: Vec4<Logic> = smallvec![Logic::X; self.port_d.h_wires.len()];
                    sim.get_wires(WireState::Cur, &self.port_d.h_wires, &mut d);
                    sim.set_wires_deferred(&self.port_q.h_wires, &mut d);
                }
            }
        }
    }
}

impl SimCell for AddCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_a, &self.port_b])
    }
    fn out_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_y])
    }

    fn simulate(&self, sim: &mut SimState) {}
}
