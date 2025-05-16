use lazy_static::lazy_static;
use util::compile_sv::compile;
use yosys_isim::model::Module;

mod test_bit_mem;
mod test_gates;
mod util;

lazy_static! {
    pub static ref TEST_GATES_SV: Vec<Module> = compile("src/test_gates.sv");
    pub static ref TEST_BIT_MEM: Vec<Module> = compile("src/test_bit_mem.sv");
}
