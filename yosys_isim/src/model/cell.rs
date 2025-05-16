use crate::model::Port;
use crate::ops::BinaryOp;
use crate::ops::TernaryOp;
use crate::ops::UnaryOp;
use crate::sim::Edge;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Clone)]
#[enum_dispatch(SimCell)]
pub enum Cell {
    UnaryOpCell(UnaryOpCell),
    BinaryOpCell(BinaryOpCell),
    TernaryOpCell(TernaryOpCell),
    DFlipFlopCell(DFlipFlopCell),
    AddCell(AddCell),
}

#[derive(Debug, Clone)]
pub struct UnaryOpCell {
    pub name: String,
    pub op: UnaryOp,
    pub port_a: Port,
    pub port_y: Port,
}

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

#[derive(Debug, Clone)]
pub struct TernaryOpCell {
    pub name: String,
    pub op: TernaryOp,
    pub port_a: Port,
    pub port_b: Port,
    pub port_c: Port,
    pub port_y: Port,
}
