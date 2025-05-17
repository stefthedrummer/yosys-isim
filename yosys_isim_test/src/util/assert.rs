#![allow(unused)]

use yosys_isim::common::Vec4;
use yosys_isim::model::Logic;
use yosys_isim::model::Logic_Len;

pub fn assert_eq<const L: usize>(a: [Logic; L], b: [Logic; L]) {
    assert!(Logic::is_slice_eq(&a, &b, Logic::is_eq_logical))
}

pub fn assert_in_out_expected(
    name: &str,
    input: &[impl Into<Logic> + Clone],
    actual: &[impl Into<Logic> + Clone],
    expected: &[impl Into<Logic> + Clone],
) {
    assert_monomorphic(
        name,
        input.iter().map(|it| it.clone().into()).collect(),
        actual.iter().map(|it| it.clone().into()).collect(),
        expected.iter().map(|it| it.clone().into()).collect(),
    );
}

fn assert_monomorphic(name: &str, input: Vec4<Logic>, actual: Vec4<Logic>, expected: Vec4<Logic>) {
    if !Logic::is_slice_eq(&actual, &expected, Logic::is_eq_physical) {
        panic!(
            "Error: {}({:?}) = {:?}, actually expected {:?}",
            name, input, actual, expected
        );
    }
}
