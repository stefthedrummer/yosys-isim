use crate::common::Vec4;
use crate::model::AddCell;
use crate::model::BinaryOpCell;
use crate::model::Cell;
use crate::model::DFlipFlopCell;
use crate::model::Port;
use crate::model::TernaryOpCell;
use crate::model::UnaryOpCell;
use crate::ops::BinaryOp;
use crate::ops::UnaryOp;
use crate::sim::Edge;
use crate::sim::Logic;
use crate::sim::SimState;
use crate::sim::StateRef;
use enum_dispatch::enum_dispatch;
use smallvec::smallvec;

#[enum_dispatch]
pub trait SimCell {
    fn name(&self) -> &str;
    fn in_ports(&self) -> Vec4<&Port>;
    fn out_ports(&self) -> Vec4<&Port>;
    fn simulate(&self, sim: &mut SimState);
}

impl SimCell for UnaryOpCell {
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

        sim.get_wires(StateRef::Cur, &self.port_a.h_wires, &mut a);

        let not = &sim.ops.unary[UnaryOp::NOT];

        for i in 0..a.len() {
            y[i] = not[a[i]];
        }

        sim.set_wires(StateRef::Cur, &self.port_y.h_wires, &y);
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

        sim.get_wires(StateRef::Cur, &self.port_a.h_wires, &mut a);
        sim.get_wires(StateRef::Cur, &self.port_b.h_wires, &mut b);

        let op = &sim.ops.binary[self.op];
        for i in 0..a.len() {
            y[i] = op[(a[i], b[i])];
        }

        sim.set_wires(StateRef::Cur, &self.port_y.h_wires, &y);
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
                    sim.get_wires(StateRef::Cur, &self.port_d.h_wires, &mut d);
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

    fn simulate(&self, sim: &mut SimState) {
        let mut a: Vec4<Logic> = smallvec![Logic::X; self.port_a.h_wires.len()];
        let mut b: Vec4<Logic> = smallvec![Logic::X; self.port_b.h_wires.len()];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.h_wires.len()];

        sim.get_wires(StateRef::Cur, &self.port_a.h_wires, &mut a);
        sim.get_wires(StateRef::Cur, &self.port_b.h_wires, &mut b);

        let xor = &sim.ops.binary[BinaryOp::XOR];
        let and = &sim.ops.binary[BinaryOp::AND];
        let or = &sim.ops.binary[BinaryOp::OR];

        let mut c = Logic::_0;

        for i in 0..y.len() {
            let a_xor_b = xor[(a[i], b[i])];
            y[i] = xor[(a_xor_b, c)];
            c = or[(and[(a_xor_b, c)], and[(a[i], b[i])])];
        }

        sim.set_wires(StateRef::Cur, &self.port_y.h_wires, &y);
    }
}

impl SimCell for TernaryOpCell {
    fn name(&self) -> &str {
        &self.name
    }
    fn in_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_a, &self.port_b, &self.port_c])
    }
    fn out_ports(&self) -> Vec4<&Port> {
        Vec4::from_slice(&[&self.port_y])
    }
    fn simulate(&self, sim: &mut SimState) {
        let mut a: Vec4<Logic> = smallvec![Logic::X; self.port_a.h_wires.len()];
        let mut b: Vec4<Logic> = smallvec![Logic::X; self.port_b.h_wires.len()];
        let mut c: Vec4<Logic> = smallvec![Logic::X; self.port_c.h_wires.len()];
        let mut y: Vec4<Logic> = smallvec![Logic::X; self.port_y.h_wires.len()];

        sim.get_wires(StateRef::Cur, &self.port_a.h_wires, &mut a);
        sim.get_wires(StateRef::Cur, &self.port_b.h_wires, &mut b);
        sim.get_wires(StateRef::Cur, &self.port_c.h_wires, &mut c);

        let op = &sim.ops.ternary[self.op];

        for i in 0..y.len() {
            y[i] = op[(a[i], b[i], c[i])];
        }

        sim.set_wires(StateRef::Cur, &self.port_y.h_wires, &y);
    }
}
