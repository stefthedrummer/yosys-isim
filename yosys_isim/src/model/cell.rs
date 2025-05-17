use crate::model::Edge;
use crate::model::HWire;
use crate::model::HWireOrLogic;
use crate::model::In;
use crate::model::Out;
use crate::model::Port;
use crate::ops::BinaryMapOp;
use crate::ops::ComparisonOp;
use crate::ops::TernaryMapOp;
use crate::ops::UnaryMapOp;
use enum_dispatch::enum_dispatch;

pub type HCell = usize;
pub type CellInPort = Port<In, HWireOrLogic>;
pub type CellOutPort = Port<Out, HWire>;

#[derive(Debug, Clone)]
#[enum_dispatch(CellSimModel)]
pub enum Cell {
    UnaryMapOpCell(UnaryMapOpCell),
    BinaryMapOpCell(BinaryMapOpCell),
    TernaryMapOpCell(TernaryMapOpCell),
    DFlipFlopCell(DFlipFlopCell),
    AddCell(AddCell),
    // ShiftCell(ShiftCell),
}

#[derive(Debug, Clone)]
pub struct UnaryMapOpCell {
    pub name: String,
    pub op: UnaryMapOp,
    pub port_a: CellInPort,
    pub port_y: CellOutPort,
}

#[derive(Debug, Clone)]
pub struct BinaryMapOpCell {
    pub name: String,
    pub op: BinaryMapOp,
    pub port_a: CellInPort,
    pub port_b: CellInPort,
    pub port_y: CellOutPort,
}

#[derive(Debug, Clone)]
pub struct TernaryMapOpCell {
    pub name: String,
    pub op: TernaryMapOp,
    pub port_a: CellInPort,
    pub port_b: CellInPort,
    pub port_c: CellInPort,
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
pub struct ComparisonOpCell {
    pub name: String,
    pub op: ComparisonOp,
    pub port_a: CellInPort,
    pub port_b: CellInPort,
    pub port_y: CellOutPort,
}

// #[derive(Debug, Clone)]
// pub struct ShiftCell {
//     pub name: String,
//     pub port_a: CellInPort,
//     pub port_b: CellInPort,
//     pub port_y: CellOutPort,
// }
