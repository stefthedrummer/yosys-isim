use std::fs::File;
use std::io::BufReader;

use crate::common::FindByName;
use crate::common::SimError;
use crate::common::Vec4;
use crate::json;
use crate::model;
use crate::sim::Edge;

use super::Connection;
use super::parse_connections;

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
    let mut in_ports: Vec4<model::Port> = Vec4::new();
    let mut out_ports: Vec4<model::Port> = Vec4::new();

    for (port_name, json_port) in json_module.ports.iter() {
        let port = model::Port {
            name: port_name.to_string(),
            h_wires: json_port.bits.clone(),
        };

        match json_port.direction {
            json::PortDirection::Input => &mut in_ports,
            json::PortDirection::Output => &mut out_ports,
        }
        .push(port);
    }

    for (cell_name, json_cell) in json_module.cells.iter() {
        let cell: model::Cell =    match json_cell.r#type {
            json::CellType::AND | json::CellType::SyntAND => {
                parse_binop(cell_name, json_cell, ("A", "B", "Y"), model::BinaryOp::AND)?
            }
            json::CellType::OR | json::CellType::SyntOR => {
                parse_binop(cell_name, json_cell, ("A", "B", "Y"), model::BinaryOp::OR)?
            }
            json::CellType::NOT => {
                    parse_not(cell_name, json_cell, ("A", "Y"))?
            }
            json::CellType::DFF => {
                parse_flipflop(cell_name, json_cell, ("CLK", "D", "Q"), None)?
            }
            json::CellType::SyncDFFPos => {
                parse_flipflop(cell_name, json_cell, ("C", "D", "Q"), Some(Edge::POSITIVE))?
            }
            json::CellType::SyntNAND => {
                parse_binop(cell_name, json_cell, ("A", "B", "Y"), model::BinaryOp::NAND)?
            }
            json::CellType::SyntNOR => {
                parse_binop(cell_name, json_cell, ("A", "B", "Y"), model::BinaryOp::NOR)?
            }
            json::CellType::SyncXOR => {
                parse_binop(cell_name, json_cell, ("A", "B", "Y"), model::BinaryOp::XOR)?
            }
            json::CellType::SyncXNOR => {
                parse_binop(cell_name, json_cell, ("A", "B", "Y"), model::BinaryOp::XNOR)?
            }
            json::CellType::Add => {
                parse_add(cell_name, json_cell, ("A", "B", "Y"))?
            }
            json::CellType::SyncOAI3 => todo!(),
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

fn parse_binop(
    cell_name: &str,
    json_cell: &json::Cell,
    connection_names: (&str, &str, &str),
    binary_op: model::BinaryOp,
) -> Result<model::Cell, SimError> {
    let connections: Vec4<Connection<'_>> = parse_connections(json_cell)?;

    let conn_a = connections.iter().find_by_name(connection_names.0)?;
    let conn_b = connections.iter().find_by_name(connection_names.1)?;
    let conn_y = connections.iter().find_by_name(connection_names.2)?;

    if conn_a.width != conn_b.width && conn_b.width != conn_y.width {
        return Err(SimError::JsonError {
            msg: format!("Widths not matching"),
        });
    }

    Ok(model::Cell::BinaryOpCell(model::BinaryOpCell {
        name: cell_name.to_string(),
        op: binary_op,
        // width: conn_y.width,
        port_a: conn_a.to_port(),
        port_b: conn_b.to_port(),
        port_y: conn_y.to_port(),
    }))
}

fn parse_not(
    cell_name: &str,
    json_cell: &json::Cell,
    connection_names: (&str, &str),
) -> Result<model::Cell, SimError> {
    let connections: Vec4<Connection<'_>> = parse_connections(json_cell)?;

    let conn_a = connections.iter().find_by_name(connection_names.0)?;
    let conn_y = connections.iter().find_by_name(connection_names.1)?;

    Ok(model::Cell::NotOpCell(model::NotOpCell {
        name: cell_name.to_string(),
        // width: conn_y.width,
        port_a: conn_a.to_port(),
        port_y: conn_y.to_port(),
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
        port_clk: conn_clk.to_port(),
        port_d: conn_d.to_port(),
        port_q: conn_q.to_port(),
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

    if conn_a.width != conn_b.width && conn_b.width != conn_y.width {
        return Err(SimError::JsonError {
            msg: format!("Widths not matching"),
        });
    }

    Ok(model::Cell::AddCell(model::AddCell {
        name: cell_name.to_string(),
        port_a: conn_a.to_port(),
        port_b: conn_b.to_port(),
        port_y: conn_y.to_port(),
    }))
}
