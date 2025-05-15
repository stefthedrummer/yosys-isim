#![allow(non_upper_case_globals)]

use enum_dispatch::enum_dispatch;

use crate::{
    common::{HasName, Vec4},
    make_enum,
    sim::Edge,
};

pub type HCell = usize;
pub type HWire = usize;

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

#[derive(Debug, Clone)]
#[enum_dispatch(SimCell)]
pub enum Cell {
    NotOpCell(NotOpCell),
    BinaryOpCell(BinaryOpCell),
    TernaryOpCell(TernaryOpCell),
    DFlipFlopCell(DFlipFlopCell),
    AddCell(AddCell),
}

#[derive(Debug, Clone)]
pub struct NotOpCell {
    pub name: String,
    pub port_a: Port,
    pub port_y: Port,
}

make_enum![enum BinaryOp repr(u8) {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
    AND_NOT,
    OR_NOT,
}];

#[derive(Debug, Clone)]
pub struct BinaryOpCell {
    pub name: String,
    pub op: BinaryOp,
    pub port_a: Port,
    pub port_b: Port,
    pub port_y: Port,
}

#[derive(Debug, Clone)]
pub struct DFlipFlopCell {
    pub name: String,
    pub polarity: Edge,
    pub port_clk: Port,
    pub port_d: Port,
    pub port_q: Port,
}

#[derive(Debug, Clone)]
pub struct AddCell {
    pub name: String,
    pub port_a: Port,
    pub port_b: Port,
    pub port_y: Port,
}

make_enum![enum TernaryOp repr(u8) {
    AND_OR_INV,
    OR_AND_INV,
}];

#[derive(Debug, Clone)]
pub struct TernaryOpCell {
    pub name: String,
    pub op: TernaryOp,
    pub port_a: Port,
    pub port_b: Port,
    pub port_c: Port,
    pub port_y: Port,
}

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
