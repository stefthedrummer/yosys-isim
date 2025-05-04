use yosys_isim::{common::Vec4, sim::Logic};

pub fn assert<I: Into<Logic> + Clone, A: Into<Logic> + Clone, E: Into<Logic> + Clone>(
    name: &str,
    input: &[I],
    actual: &[A],
    expected: &[E],
) {
    assert_monomorphic(
        name,
        input.iter().map(|it| it.clone().into()).collect(),
        actual.iter().map(|it| it.clone().into()).collect(),
        expected.iter().map(|it| it.clone().into()).collect(),
    );
}

fn assert_monomorphic(name: &str, input: Vec4<Logic>, actual: Vec4<Logic>, expected: Vec4<Logic>) {
    if actual != expected {
        panic!(
            "Error: {}({:?}) = {:?}, actually expected {:?}",
            name, input, actual, expected
        );
    }
}
