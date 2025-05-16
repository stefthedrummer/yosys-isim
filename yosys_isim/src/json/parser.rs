use crate::common::FindByName;
use crate::common::SimError;
use crate::common::Vec4;
use crate::json;
use crate::json::Connection;
use crate::json::parse_connections;
use crate::model;
use crate::model::HWireOrLogic;
use crate::model::ModuleInPort;
use crate::model::ModuleOutPort;
use crate::ops;
use crate::sim::Edge;
use crate::sim::Logic;
use serde_json::Value;
use smallvec::smallvec;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;

pub fn parse_modules_from_file(file_name: &str) -> Result<Vec<model::Module>, SimError> {
    parse_netlist(&serde_json::from_reader(BufReader::new(File::open(
        file_name,
    )?))?)
}

pub fn parse_netlist(json_module: &json::Netlist) -> Result<Vec<model::Module>, SimError> {
    json_module
        .modules
        .iter()
        .map(|(name, json_module)| parse_module(name, json_module))
        .collect()
}

#[rustfmt::skip]
fn parse_module(name: &str, json_module: &json::Module) -> Result<model::Module, SimError> {
    let mut cells: Vec<model::Cell> = Vec::new();
    let mut in_ports: Vec4<model::ModuleInPort> = Vec4::new();
    let mut out_ports: Vec4<model::ModuleOutPort> = Vec4::new();

    for (port_name, json_port) in json_module.ports.iter() {
        let name: String = port_name.to_string();
        let wires = HWireOrLogic::only_HWires(&parse_wires(&json_port.bits)?)?;
        match json_port.direction {
            json::PortDirection::Input => in_ports.push(ModuleInPort {
                name,
                wires ,
                dir: PhantomData,
            }),
            json::PortDirection::Output => out_ports.push(ModuleOutPort {
                name,
                wires ,
                dir: PhantomData,
            }),
        };
    }


    for (cell_name, json_cell) in json_module.cells.iter() {
        let cell: model::Cell =  match json_cell.r#type {
            json::CellType::AND | json::CellType::SyntAND => {
                parse_binary(cell_name, json_cell, ("A", "B", "Y"), ops::BinaryOp::AND)?
            }
            json::CellType::OR | json::CellType::SyntOR => {
                parse_binary(cell_name, json_cell, ("A", "B", "Y"), ops::BinaryOp::OR)?
            }
            json::CellType::NOT | json::CellType::SyntNOT => {
                parse_unary(cell_name, json_cell, ("A", "Y"), ops::UnaryOp::NOT)?
            }
            json::CellType::DFF => {
                parse_flipflop(cell_name, json_cell, ("CLK", "D", "Q"), None)?
            }
            json::CellType::Add => {
                parse_add(cell_name, json_cell, ("A", "B", "Y"))?
            }
            json::CellType::SyntDFFPos => {
                parse_flipflop(cell_name, json_cell, ("C", "D", "Q"), Some(Edge::POSITIVE))?
            }
            json::CellType::SyntNAND => {
                parse_binary(cell_name, json_cell, ("A", "B", "Y"), ops::BinaryOp::NAND)?
            }
            json::CellType::SyntNOR => {
                parse_binary(cell_name, json_cell, ("A", "B", "Y"), ops::BinaryOp::NOR)?
            }
            json::CellType::SyntXOR => {
                parse_binary(cell_name, json_cell, ("A", "B", "Y"), ops::BinaryOp::XOR)?
            }
            json::CellType::SyntXNOR => {
                parse_binary(cell_name, json_cell, ("A", "B", "Y"), ops::BinaryOp::XNOR)?
            }
            json::CellType::SyntAND_NOT => {
                parse_binary(cell_name, json_cell, ("A", "B", "Y"), ops::BinaryOp::AND_NOT)?
            }
            json::CellType::SyntOR_NOT => {
                parse_binary(cell_name, json_cell, ("A", "B",  "Y"), ops::BinaryOp::OR_NOT)?
            }

            json::CellType::SyntAOI3 => {
                parse_ternary(cell_name, json_cell, ("A", "B", "C", "Y"), ops::TernaryOp::AND_OR_INV)?
            }
            json::CellType::SyntOAI3 => {
                parse_ternary(cell_name, json_cell, ("A", "B", "C", "Y"), ops::TernaryOp::OR_AND_INV)?
            }
        };

        cells.push(cell);
    }

    Ok(model::Module {
        name: name.to_string(),
        cells,
        in_ports,
        out_ports,
    })
}

#[allow(non_snake_case)]
pub(super) fn parse_wires(json_wires: &Vec4<Value>) -> Result<Vec4<model::HWireOrLogic>, SimError> {
    let mut wires = smallvec![model::HWireOrLogic::Logic( Logic::X) ; json_wires.len()];
    for i in 0..wires.len() {
        wires[i] = match &json_wires[i] {
            Value::Number(h_wire) => model::HWireOrLogic::HWire(h_wire.as_u64().unwrap() as usize),
            Value::String(logic) => match Logic::from_str(&logic) {
                Some(logic) => model::HWireOrLogic::Logic(logic),
                None => {
                    return Err(SimError::JsonError {
                        msg: format!("illegal wire constant [{}]", json_wires[i]),
                    });
                }
            },
            _ => {
                return Err(SimError::JsonError {
                    msg: format!("illegal wire [{}]", json_wires[i]),
                });
            }
        }
    }
    Ok(wires)
}

fn parse_binary(
    cell_name: &str,
    json_cell: &json::Cell,
    connection_names: (&str, &str, &str),
    op: ops::BinaryOp,
) -> Result<model::Cell, SimError> {
    let connections: Vec4<Connection<'_>> = parse_connections(json_cell)?;

    let conn_a = connections.iter().find_by_name(connection_names.0)?;
    let conn_b = connections.iter().find_by_name(connection_names.1)?;
    let conn_y = connections.iter().find_by_name(connection_names.2)?;

    if conn_a.width != conn_b.width || conn_b.width != conn_y.width {
        return Err(SimError::JsonError {
            msg: format!("Widths not matching"),
        });
    }

    Ok(model::Cell::BinaryOpCell(model::BinaryOpCell {
        name: cell_name.to_string(),
        op,
        // width: conn_y.width,
        port_a: conn_a.to_in_port()?,
        port_b: conn_b.to_in_port()?,
        port_y: conn_y.to_out_port()?,
    }))
}

fn parse_unary(
    cell_name: &str,
    json_cell: &json::Cell,
    connection_names: (&str, &str),
    op: ops::UnaryOp,
) -> Result<model::Cell, SimError> {
    let connections: Vec4<Connection<'_>> = parse_connections(json_cell)?;

    let conn_a = connections.iter().find_by_name(connection_names.0)?;
    let conn_y = connections.iter().find_by_name(connection_names.1)?;

    Ok(model::Cell::UnaryOpCell(model::UnaryOpCell {
        name: cell_name.to_string(),
        op: op,
        port_a: conn_a.to_in_port()?,
        port_y: conn_y.to_out_port()?,
    }))
}

fn parse_flipflop(
    cell_name: &str,
    json_cell: &json::Cell,
    connection_names: (&str, &str, &str),
    polarity: Option<Edge>,
) -> Result<model::Cell, SimError> {
    let connections: Vec4<Connection<'_>> = parse_connections(json_cell)?;

    let conn_clk = connections.iter().find_by_name(connection_names.0)?;
    let conn_d = connections.iter().find_by_name(connection_names.1)?;
    let conn_q = connections.iter().find_by_name(connection_names.2)?;

    if conn_d.width != conn_q.width {
        return Err(SimError::JsonError {
            msg: format!("Widths not matching"),
        });
    }

    Ok(model::Cell::DFlipFlopCell(model::DFlipFlopCell {
        name: cell_name.to_string(),
        polarity: polarity.unwrap_or(if conn_clk.polarity > 0 {
            Edge::POSITIVE
        } else {
            Edge::NEGATIVE
        }),
        port_clk: conn_clk.to_in_port()?,
        port_d: conn_d.to_in_port()?,
        port_q: conn_q.to_out_port()?,
    }))
}

fn parse_add(
    cell_name: &str,
    json_cell: &json::Cell,
    connection_names: (&str, &str, &str),
) -> Result<model::Cell, SimError> {
    let connections: Vec4<Connection<'_>> = parse_connections(json_cell)?;

    let conn_a = connections.iter().find_by_name(connection_names.0)?;
    let conn_b = connections.iter().find_by_name(connection_names.1)?;
    let conn_y = connections.iter().find_by_name(connection_names.2)?;

    if conn_a.width != conn_b.width || conn_b.width != conn_y.width {
        return Err(SimError::JsonError {
            msg: format!("Widths not matching"),
        });
    }

    Ok(model::Cell::AddCell(model::AddCell {
        name: cell_name.to_string(),
        port_a: conn_a.to_in_port()?,
        port_b: conn_b.to_in_port()?,
        port_y: conn_y.to_out_port()?,
    }))
}

fn parse_ternary(
    cell_name: &str,
    json_cell: &json::Cell,
    connection_names: (&str, &str, &str, &str),
    op: ops::TernaryOp,
) -> Result<model::Cell, SimError> {
    let connections: Vec4<Connection<'_>> = parse_connections(json_cell)?;

    let conn_a = connections.iter().find_by_name(connection_names.0)?;
    let conn_b = connections.iter().find_by_name(connection_names.1)?;
    let conn_c = connections.iter().find_by_name(connection_names.2)?;
    let conn_y = connections.iter().find_by_name(connection_names.3)?;

    if conn_a.width != conn_b.width || conn_b.width != conn_c.width || conn_c.width != conn_y.width
    {
        return Err(SimError::JsonError {
            msg: format!("Widths not matching"),
        });
    }

    Ok(model::Cell::TernaryOpCell(model::TernaryOpCell {
        name: cell_name.to_string(),
        op: op,
        port_a: conn_a.to_in_port()?,
        port_b: conn_b.to_in_port()?,
        port_c: conn_c.to_in_port()?,
        port_y: conn_y.to_out_port()?,
    }))
}
