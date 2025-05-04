use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::Vec4;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Netlist {
    pub modules: HashMap<String, Module>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module {
    pub ports: HashMap<String, Port>,
    pub cells: HashMap<String, Cell>,
    pub netnames: HashMap<String, Net>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Port {
    pub direction: PortDirection,
    pub bits: Vec4<usize>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PortDirection {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "output")]
    Output,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cell {
    pub r#type: CellType,
    pub port_directions: HashMap<String, PortDirection>,
    pub connections: HashMap<String, Vec4<usize>>,
    pub parameters: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]

pub enum CellType {
    #[serde(rename = "$and")]
    AND,
    #[serde(rename = "$or")]
    OR,
    #[serde(rename = "$not")]
    NOT,
    #[serde(rename = "$dff")]
    DFF,
    #[serde(rename = "$add")]
    Add,
    // --------------------------------
    #[serde(rename = "$_AND_")]
    SyntAND,
    #[serde(rename = "$_OR_")]
    SyntOR,
    #[serde(rename = "$_XOR_")]
    SyncXOR,
    // --------------------------------
    #[serde(rename = "$_NAND_")]
    SyntNAND,
    #[serde(rename = "$_NOR_")]
    SyntNOR,
    #[serde(rename = "$_XNOR_")]
    SyncXNOR,
    // --------------------------------
    #[serde(rename = "$_DFF_P_")]
    SyncDFFPos,
    // --------------------------------
    #[serde(rename = "$_OAI3_")]
    SyncOAI3,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Net {
    pub bits: Vec4<usize>,
}
