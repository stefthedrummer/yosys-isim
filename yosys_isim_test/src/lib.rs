use yosys_isim::model::Module;
use lazy_static::lazy_static;
use util::compile_sv::compile;

mod test_gates;
mod util;

lazy_static! {
    pub static ref TEST_GATES_SV: Vec<Module> = compile("src/test_gates.sv");
}
