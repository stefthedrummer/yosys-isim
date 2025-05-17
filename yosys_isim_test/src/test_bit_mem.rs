#![allow(unused_imports)]

use crate::TEST_BIT_MEM;
use std::ops::Deref;
use yosys_isim::common::FindByName;
use yosys_isim::common::SimError;
use yosys_isim::sim::Sim;

#[test]
pub fn foo() {
    || -> Result<(), SimError> {
        let module = TEST_BIT_MEM.deref().iter().find_by_name("test_bit_mem")?;
        let clk = module.get_in_port::<1>("clk")?;
        let write_enable = module.get_in_port::<1>("write_enable")?;
        let addr = module.get_in_port::<4>("addr")?;
        let data_in = module.get_in_port::<1>("data_in")?;
        let data_out = module.get_out_port::<1>("data_out")?;
        let mut sim = Sim::new(&module);

        sim.set(&addr, [1, 0, 0, 0]);
        sim.set(&data_in, [1]);
        sim.simulate()?;

        sim.set(&write_enable, [1]);
        sim.simulate()?;
        sim.set(&write_enable, [0]);

        Ok(())
    }()
    .unwrap()
}
