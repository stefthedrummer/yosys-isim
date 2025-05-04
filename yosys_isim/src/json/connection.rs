use crate::{
    common::{HasName, SimError, Vec4},
    model::{self, HWire},
};

use super::json;

pub(super) struct Connection<'a> {
    pub name: &'a String,
    pub wires: &'a Vec4<HWire>,
    pub direction: json::PortDirection,
    pub signed: bool,
    pub polarity: u32,
    pub width: u32,
}

impl<'a> HasName for Connection<'a> {
    fn name(&self) -> &'a str {
        &self.name
    }
}

impl<'a> Connection<'a> {
    pub(super) fn to_port(&self) -> model::Port {
        model::Port {
            name: self.name.to_string(),
            h_wires: self.wires.clone(),
        }
    }
}

pub(super) fn parse_connections(json_cell: &json::Cell) -> Result<Vec4<Connection>, SimError> {
    let mut connections: Vec4<Connection> = Vec4::new();

    for (name, wires) in json_cell.connections.iter() {
        connections.push(Connection {
            name: &name,
            direction: json::PortDirection::Input,
            wires: &wires,
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
