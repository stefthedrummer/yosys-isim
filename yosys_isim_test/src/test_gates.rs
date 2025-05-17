use crate::TEST_GATES_SV;
use crate::util::assert::assert;
use std::ops::Deref;
use yosys_isim::common::FindByName;
use yosys_isim::common::SimError;
use yosys_isim::model::Logic;
use yosys_isim::sim::Sim;

#[test]
pub fn test_and() {
    do_test_binary_op::<1>("And", |a, b| a & b).unwrap();
}

#[test]
pub fn test_and2() {
    do_test_binary_op::<32>("And32", |a, b| a & b).unwrap();
}

#[test]
pub fn test_or() {
    do_test_binary_op::<1>("Or", |a, b| a | b).unwrap();
}

#[test]
pub fn test_or2() {
    do_test_binary_op::<32>("Or32", |a, b| a | b).unwrap();
}

#[test]
pub fn test_nand() {
    do_test_binary_op::<1>("Nand", |a, b| !(a & b)).unwrap();
}

#[test]
pub fn test_nor() {
    do_test_binary_op::<1>("Nor", |a, b| !(a | b)).unwrap();
}

#[allow(unused)]
pub fn do_test_binary_op<const L: usize>(
    module_name: &str,
    eval: fn(bool, bool) -> bool,
) -> Result<(), SimError> {
    let module = TEST_GATES_SV.deref().iter().find_by_name(module_name)?;

    for (a, b) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
        let port_a = module.get_in_port::<L>("a")?;
        let port_b = module.get_in_port::<L>("b")?;
        let port_y = module.get_out_port::<L>("y")?;
        let mut sim = Sim::new(&module);

        sim.set(&port_a, [a; L]);
        sim.set(&port_b, [b; L]);
        sim.simulate()?;
        let y: [Logic; L] = sim.get(&port_y);

        assert(module_name, &[a, b], &y, &[eval(a > 0, b > 0); L]);
    }

    Ok(())
}

#[test]
pub fn test_dff() {
    (|| -> Result<(), SimError> {
        let module = TEST_GATES_SV.deref().iter().find_by_name("Dff")?;

        let port_c = module.get_in_port::<1>("c")?;
        let port_d = module.get_in_port::<2>("d")?;
        let port_q = module.get_out_port::<2>("q")?;
        let mut sim: Sim<'_> = Sim::new(&module);

        assert_eq!(sim.get(&port_q), [Logic::X; 2]);

        sim.set(&port_c, [0]);
        sim.set(&port_d, [1, 0]);
        sim.simulate()?;

        assert_eq!(sim.get(&port_q), [Logic::X; 2]);

        sim.set(&port_c, [1]);
        sim.simulate()?;

        assert_eq!(sim.get(&port_q), [Logic::_1, Logic::_0]);

        sim.set(&port_c, [0]);
        sim.set(&port_d, [0, 1]);
        sim.simulate()?;

        sim.set(&port_c, [1]);
        sim.simulate()?;

        assert_eq!(sim.get(&port_q), [Logic::_0, Logic::_1]);

        Ok(())
    })()
    .unwrap()
}

#[test]
pub fn test_add() {
    (|| -> Result<(), SimError> {
        let module = TEST_GATES_SV.deref().iter().find_by_name("Add")?;

        let port_a = module.get_in_port::<8>("a")?;
        let port_b = module.get_in_port::<8>("b")?;
        let port_y = module.get_out_port::<8>("y")?;
        let mut sim: Sim<'_> = Sim::new(&module);

        for int_a in 0..15 {
            for int_b in 0..15 {
                let a = Logic::to_bits::<8>(int_a);
                let b = Logic::to_bits::<8>(int_b);
                let expected = Logic::to_bits::<8>(int_a + int_b);

                sim.set(&port_a, a);
                sim.set(&port_b, b);
                sim.simulate()?;

                assert_eq!(sim.get(&port_y), expected);
            }
        }

        Ok(())
    })()
    .unwrap()
}
