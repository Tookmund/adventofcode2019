mod common;
use common::*;

#[test]
fn day9_quine() {
    testprogram_io(
        vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ],
        None,
        None,
        Some(b"109\n1\n204\n-1\n1001\n100\n1\n100\n1008\n100\n16\n101\n1006\n101\n0\n99\n"),
    );
}

#[test]
fn day9_16digit() {
    testprogram_io(
        vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0],
        None,
        None,
        Some(b"1219070632396864\n"),
    );
}

#[test]
fn day9_large_number() {
    testprogram_io(
        vec![104, 1125899906842624, 99],
        None,
        None,
        Some(b"1125899906842624\n"),
    )
}

#[test]
fn day9_rel_base() {
    testprogram_io(
        vec![3, 1985, 109, 2000, 109, 19, 204, -34, 99],
        None,
        Some(b"1985"),
        Some(b"1985\n"),
    )
}

#[test]
fn day9_rel_base_add() {
    testprogram_io(
        vec![109, 4, 2101, 30, -3, 0, 99],
        Some(vec![34, 4, 2101, 30, -3, 0, 99]),
        None,
        None,
    )
}
