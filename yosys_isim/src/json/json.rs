use crate::common::Str8;
use crate::common::Vec4;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Netlist {
    pub modules: HashMap<Str8, Module>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Module {
    pub ports: HashMap<Str8, Port>,
    pub cells: HashMap<Str8, Cell>,
    pub netnames: HashMap<Str8, Net>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Port {
    pub direction: PortDirection,
    pub bits: Vec4<Value>,
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
    pub r#type: Str8,
    pub port_directions: HashMap<Str8, PortDirection>,
    pub connections: HashMap<Str8, Vec4<Value>>,
    pub parameters: HashMap<Str8, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Net {
    pub bits: Vec4<usize>,
}
