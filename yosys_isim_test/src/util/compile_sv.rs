use std::{fs, process::Command};
use yosys_isim::{common::SimError, json::parse_modules_from_file, model::Module};

pub fn compile(sv_file: &str) -> Vec<Module> {
    match do_compile(sv_file) {
        Ok(modules) => modules,
        Err(err) => panic!("{:?}", err),
    }
}

fn do_compile(sv_file: &str) -> Result<Vec<Module>, SimError> {
    let netlist_file = sv_file.replace("/", "_");

    fs::create_dir_all("target")?;

    let mut yosys = Command::new("yosys");
    yosys.args([
        "-p",
        &format!(
            "read_verilog -sv {}; proc; flatten; opt; synth; write_json target/{}.json;",
            sv_file, netlist_file
        ),
    ]);
    yosys.spawn()?.wait()?;

    parse_modules_from_file(&format!("target/{}.json", netlist_file))
}
