use crate::common::Str8;
use crate::common::Vec4;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

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
    pub port_directions: HashMap<String, PortDirection>,
    pub connections: HashMap<String, Vec4<Value>>,
    pub parameters: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Net {
    pub bits: Vec4<usize>,
}
