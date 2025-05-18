use crate::common::HasName;
use crate::common::SimError;
use crate::common::Vec4;
use crate::json;
use crate::json::parse_wires;
use crate::model::HWireOrLogic;
use crate::model::{self};
use small_str::ToSmallStr;
use std::marker::PhantomData;

pub(super) struct Connection<'a> {
    pub name: &'a str,
    pub wires: Vec4<HWireOrLogic>,
    pub direction: json::PortDirection,
    pub signed: bool,
    pub polarity: u32,
    pub width: u32,
}

impl<'a> HasName for Connection<'a> {
    const LABEL: &'static str = "connection";
    fn name(&self) -> &'a str {
        &self.name
    }
}

impl<'a> Connection<'a> {
    pub(super) fn to_in_port<const L: usize>(&self) -> Result<model::CellInPort<L>, SimError> {
        if L != 0 && self.wires.len() != L {
            return Err(SimError::IllegalState {
                msg: format!("expected width [{}] but was [{}]", L, self.wires.len()),
            });
        }

        Ok(model::CellInPort {
            name: self.name.to_smallstr(),
            wires: self.wires.clone(),
            dir: PhantomData,
        })
    }

    pub(super) fn to_out_port(&self) -> Result<model::CellOutPort, SimError> {
        Ok(model::CellOutPort {
            name: self.name.to_smallstr(),
            wires: HWireOrLogic::only_HWires(&self.wires)?,
            dir: PhantomData,
        })
    }
}

pub(super) fn parse_connections(json_cell: &json::Cell) -> Result<Vec4<Connection>, SimError> {
    let mut connections: Vec4<Connection> = Vec4::new();

    for (name, wires) in json_cell.connections.iter() {
        connections.push(Connection {
            name: &name,
            direction: json::PortDirection::Input,
            wires: parse_wires(&wires)?,
            signed: false,
            polarity: 1,
            width: 1,
        });
    }

    for (name, direction) in json_cell.port_directions.iter() {
        match connections.iter_mut().find(|it| it.name.eq(name)) {
            Some(connection) => connection.direction = *direction,
            None => {
                return Err(SimError::JsonError {
                    msg: format!("Could not assign direction for [{}]", name.to_string()),
                });
            }
        }
    }

    for (parameter, value) in json_cell.parameters.iter() {
        let index_of_underscore = match parameter.find('_') {
            Some(index) => index,
            None => match parameter.as_str() {
                "WIDTH" => continue,
                _ => {
                    return Err(SimError::JsonError {
                        msg: format!(
                            "could not recognize port name of property [{}]",
                            parameter.to_string()
                        ),
                    });
                }
            },
        };

        let connection_name = &parameter[0..index_of_underscore];
        let parameter_name = &parameter[index_of_underscore + 1..];

        let connection = match connections
            .iter_mut()
            .find(|it| it.name.eq(connection_name))
        {
            Some(connection) => connection,
            None => {
                return Err(SimError::JsonError {
                    msg: format!(
                        "could not find connection with name [{}]",
                        connection_name.to_string()
                    ),
                });
            }
        };

        match parameter_name {
            "SIGNED" => {
                connection.signed = match value.as_u64() {
                    Some(0) => false,
                    Some(1) => true,
                    _ => {
                        return Err(SimError::JsonError {
                            msg: format!(
                                "encountered illegal SIGNED-property [{}={}]",
                                parameter_name,
                                value.to_string()
                            ),
                        });
                    }
                }
            }
            "WIDTH" => {
                connection.width = match value.as_u64() {
                    Some(n) => n as u32,
                    _ => {
                        return Err(SimError::JsonError {
                            msg: format!(
                                "encountered illegal WIDTH-property [{}={}]",
                                parameter_name,
                                value.to_string()
                            ),
                        });
                    }
                }
            }
            "POLARITY" => {
                connection.polarity = match value.as_u64() {
                    Some(n) => n as u32,
                    _ => {
                        return Err(SimError::JsonError {
                            msg: format!(
                                "encountered illegal POLARITY-property [{}={}]",
                                parameter_name,
                                value.to_string()
                            ),
                        });
                    }
                }
            }
            _ => {
                return Err(SimError::JsonError {
                    msg: format!(
                        "encountered unknown property [{}={}]",
                        parameter_name,
                        value.to_string()
                    ),
                });
            }
        };
    }

    Ok(connections)
}
