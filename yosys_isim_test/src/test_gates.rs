use crate::TEST_GATES_SV;
use crate::util::assert::assert;
use std::ops::Deref;
use yosys_isim::common::FindByName;
use yosys_isim::common::SimError;
use yosys_isim::sim::Logic;
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
pub fn do_test_binary_op<const W: usize>(
    module_name: &str,
    eval: fn(bool, bool) -> bool,
) -> Result<(), SimError> {
    let module = TEST_GATES_SV.deref().iter().find_by_name(module_name)?;

    for (a, b) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
        let mut sim = Sim::new(&module);
        let port_a = sim.get_port::<W>("a")?;
        let port_b = sim.get_port::<W>("b")?;
        let port_y = sim.get_port::<W>("y")?;

        sim.set(&port_a, [a; W]);
        sim.set(&port_b, [b; W]);
        sim.simulate()?;
        let y: [Logic; W] = sim.get(&port_y);

        assert(module_name, &[a, b], &y, &[eval(a > 0, b > 0); W]);
    }

    Ok(())
}

#[test]
pub fn test_dff() {
    (|| -> Result<(), SimError> {
        let module = TEST_GATES_SV.deref().iter().find_by_name("Dff")?;

        let mut sim: Sim<'_> = Sim::new(&module);
        let port_c = sim.get_port::<1>("c")?;
        let port_d = sim.get_port::<2>("d")?;
        let port_q = sim.get_port::<2>("q")?;

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

        let mut sim: Sim<'_> = Sim::new(&module);
        let port_a = sim.get_port::<8>("a")?;
        let port_b = sim.get_port::<8>("b")?;
        let port_y = sim.get_port::<8>("y")?;

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
