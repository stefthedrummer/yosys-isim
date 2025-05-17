use crate::model::Edge;
use crate::model::HWire;
use crate::model::HWireOrLogic;
use crate::model::In;
use crate::model::Out;
use crate::model::Port;
use crate::ops::BinaryOp;
use crate::ops::TernaryOp;
use crate::ops::UnaryOp;
use enum_dispatch::enum_dispatch;

pub type HCell = usize;
pub type CellInPort = Port<In, HWireOrLogic>;
pub type CellOutPort = Port<Out, HWire>;

#[derive(Debug, Clone)]
#[enum_dispatch(CellSimModel)]
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
    pub port_a: CellInPort,
    pub port_y: CellOutPort,
}

#[derive(Debug, Clone)]
pub struct BinaryOpCell {
    pub name: String,
    pub op: BinaryOp,
    pub port_a: CellInPort,
    pub port_b: CellInPort,
    pub port_y: CellOutPort,
}

#[derive(Debug, Clone)]
pub struct DFlipFlopCell {
    pub name: String,
    pub polarity: Edge,
    pub port_clk: CellInPort,
    pub port_d: CellInPort,
    pub port_q: CellOutPort,
}

#[derive(Debug, Clone)]
pub struct AddCell {
    pub name: String,
    pub port_a: CellInPort,
    pub port_b: CellInPort,
    pub port_y: CellOutPort,
}

#[derive(Debug, Clone)]
pub struct TernaryOpCell {
    pub name: String,
    pub op: TernaryOp,
    pub port_a: CellInPort,
    pub port_b: CellInPort,
    pub port_c: CellInPort,
    pub port_y: CellOutPort,
}
