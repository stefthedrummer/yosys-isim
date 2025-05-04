#![allow(dead_code)]
mod error;

use error::*;

use napi_derive::napi;
use yosys_isim as core;
use yosys_isim::json;

#[napi]
pub struct Module {
    pub name: String,
    ptr: *mut core::Module,
}

#[napi]
impl Module {
    #[napi]
    pub fn parse_modules_from_file(file_name: String) -> Result<Vec<Module>, JsError> {
        Ok(json::parse_modules_from_file(&file_name)?
            .into_iter()
            .map(|module| Module {
                name: module.name.to_string(),
                ptr: Box::into_raw(Box::new(module)),
            })
            .collect())
    }
}

#[napi]
pub struct Sim {
    ptr: *mut core::Sim<'static>,
}

#[napi]
impl Sim {
    #[napi]
    pub fn create(module: &Module) -> Sim {
        unsafe {
            let sim: yosys_isim::Sim<'static> = core::Sim::new(&*module.ptr);
            Sim {
                ptr: Box::into_raw(Box::new(sim)),
            }
        }
    }

    #[napi]
    pub fn set(
        &self,
        port_name: String,
        #[napi(ts_arg_type = "[0 | 1]")] logics: Vec<i32>,
    ) -> Result<(), JsError> {
        unsafe {
            (*self.ptr).set_raw(&port_name, &logics)?;
            Ok(())
        }
    }

    #[napi(ts_return_type = "[0 | 1 | -1]")]
    pub fn get(&self, port_name: String, width: u32) -> Result<Vec<i32>, JsError> {
        unsafe {
            let logics = (*self.ptr).get_raw(&port_name, width as usize)?;
            Ok(logics
                .into_iter()
                .map(|it| match it {
                    yosys_isim::Logic::_0 => 0,
                    yosys_isim::Logic::_1 => 1,
                    yosys_isim::Logic::X => -1,
                })
                .collect())
        }
    }

    #[napi]
    pub fn simulate(&self) -> Result<(), JsError> {
        unsafe {
            (*self.ptr).simulate()?;
            Ok(())
        }
    }
}
