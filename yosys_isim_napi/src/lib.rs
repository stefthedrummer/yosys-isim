#![allow(dead_code)]
mod error;

use error::*;
use napi::Env;
use napi_derive::napi;
use yosys_isim::json;
use yosys_isim::model;
use yosys_isim::sim;

#[napi]
pub struct Module {
    pub name: String,
    p_module: *mut model::Module,
}

#[napi]
impl Module {
    #[napi]
    pub fn parse_modules_from_file(file_name: String) -> Result<Vec<Module>, JsError> {
        Ok(json::parse_modules_from_file(&file_name)?
            .into_iter()
            .map(|module| Module {
                name: module.name.to_string(),
                p_module: Box::into_raw(Box::new(module)),
            })
            .collect())
    }
}

#[napi]
pub struct Sim {
    p_module: *mut model::Module,
    p_sim: *mut sim::Sim<'static>,
}

#[napi]
impl Sim {
    #[napi]
    pub fn create(module: &Module) -> Sim {
        unsafe {
            let sim: sim::Sim<'static> = sim::Sim::new(&*module.p_module);
            Sim {
                p_module: module.p_module,
                p_sim: Box::into_raw(Box::new(sim)),
            }
        }
    }

    #[napi]
    pub fn set(
        &self,
        port_name: String,
        #[napi(ts_arg_type = "[0 | 1]")] logics: Vec<i64>,
    ) -> Result<(), JsError> {
        unsafe {
            let port = (*self.p_module).get_port_dynamic(
                &(*self.p_module).in_ports,
                &port_name,
                logics.len(),
            )?;

            (*self.p_sim).set_dynamic(port, &logics);
            Ok(())
        }
    }

    #[napi(ts_return_type = "[0 | 1 | -1]")]
    pub fn get(
        &self,
        env: Env,
        port_name: String,
        width: i64,
    ) -> Result<Vec<napi::JsUnknown>, JsError> {
        unsafe {
            let port = (*self.p_module).get_port_dynamic(
                &(*self.p_module).out_ports,
                &port_name,
                width as usize,
            )?;
            Ok((*self.p_sim)
                .get_dynamic(port)
                .into_iter()
                .map(|logic| match logic {
                    sim::Logic::_0 => env.create_uint32(0).unwrap().into_unknown(),
                    sim::Logic::_1 => env.create_uint32(1).unwrap().into_unknown(),
                    sim::Logic::X => env.create_string("X").unwrap().into_unknown(),
                })
                .collect())
        }
    }

    #[napi]
    pub fn simulate(&self) -> Result<(), JsError> {
        unsafe {
            (*self.p_sim).simulate()?;
            Ok(())
        }
    }
}
