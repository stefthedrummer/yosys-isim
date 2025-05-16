// #![allow(unused_imports)]

// use crate::TEST_BIT_MEM;
// use std::ops::Deref;
// use yosys_isim::{FindByName, Sim, SimError};

// #[test]
// pub fn foo() {
//     || -> Result<(), SimError> {
//         let module = TEST_BIT_MEM.deref().iter().find_by_name("test_bit_mem")?;
//         let mut sim = Sim::new(&module);

//         let clk = sim.get_port::<1>("clk")?;
//         let write_enable = sim.get_port::<1>("write_enable")?;
//         let addr = sim.get_port::<4>("addr")?;
//         let data_in = sim.get_port::<1>("data_in")?;
//         let data_out = sim.get_port::<1>("data_out")?;

//         sim.set(&addr, [1, 0, 0, 0]);
//         sim.set(&data_in, [1]);
//         sim.simulate()?;

//         sim.set(&write_enable, [1]);
//         sim.simulate()?;
//         sim.set(&write_enable, [0]);

//         Ok(())
//     }()
//     .unwrap()
// }
