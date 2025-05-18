use crate::Logic;
use crate::Logic_Variants;
use crate::LogicArray64;
use crate::LogicSlice;
use more_collections::SmallSet;
use smallvec::SmallVec;
use smallvec::smallvec;

type L = Logic;

#[test]
fn test_conversion() {
    assert_eq!(
        RANDOM_LOGICS,
        LogicArray64::to_logics_fixed::<64>(&LogicArray64::from_logics(&RANDOM_LOGICS))
    )
}

#[test]
fn test_ops() {
    let ops: &[(
        &'static str,
        fn(&LogicArray64, &LogicArray64) -> LogicArray64,
        fn(bool, bool) -> bool,
    )] = &[
        ("and", LogicArray64::and, |a, b| a & b),
        ("or", LogicArray64::or, |a, b| a | b),
    ];

    for (name, op, reference_op) in ops {
        for a in Logic_Variants {
            for b in Logic_Variants {
                let actual_res = wrap_op(op)(a, b);
                let expected_res = wrap_reference_op(reference_op)(a, b);
                println!(
                    "{}({:?},{:?}) = {:?}, expected {:?}",
                    name, a, b, actual_res, expected_res
                );
                assert_eq!(actual_res, expected_res)
            }
        }
    }
}

#[test]
fn test_pack_unpack() {
    let packged = LogicArray64::from_logics(&RANDOM_LOGICS);
    let mut unpacked = [Logic::X; 64];

    LogicArray64::to_logics(&packged, &mut unpacked);

    assert_eq!(unpacked, RANDOM_LOGICS)
}

impl Logic {
    fn to_bool_set(&self) -> SmallVec<[bool; 2]> {
        match self {
            Logic::_0 => smallvec![false],
            Logic::_1 => smallvec![true],
            Logic::X => smallvec![false, true],
        }
    }
}

const fn wrap_op(
    op: impl Fn(&LogicArray64, &LogicArray64) -> LogicArray64,
) -> impl Fn(Logic, Logic) -> Logic {
    move |a: Logic, b: Logic| -> Logic {
        LogicArray64::to_logic(&op(
            &LogicArray64::from_logic(a),
            &LogicArray64::from_logic(b),
        ))
    }
}

const fn wrap_reference_op(op: impl Fn(bool, bool) -> bool) -> impl Fn(Logic, Logic) -> Logic {
    move |a: Logic, b: Logic| -> Logic {
        let mut y_bool: SmallSet<bool, 2> = SmallSet::new();
        for a_bool in a.to_bool_set() {
            for b_bool in b.to_bool_set() {
                y_bool.insert(op(a_bool, b_bool));
            }
        }
        match (y_bool.contains(&false), y_bool.contains(&true)) {
            (true, false) => Logic::_0,
            (false, true) => Logic::_1,
            (true, true) => Logic::X,
            (false, false) => panic!("illegal state"),
        }
    }
}

static RANDOM_LOGICS: [L; 64] = [
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
    L::_1,
    L::X,
    L::_0,
];
