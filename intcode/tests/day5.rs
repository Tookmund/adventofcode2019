mod common;
use common::*;

#[test]
fn day5_part1_1() {
    testprogram(vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99])
}

#[test]
fn day5_part1_2_negative() {
    testprogram(vec![1101, 100, -1, 4, 0], vec![1101, 100, -1, 4, 99])
}

#[test]
fn day5_part2_position_equal() {
    testprogram_io(
        vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
        None,
        Some(b"8\n"),
        Some(b"1\n"),
    );
    testprogram_io(
        vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
        None,
        Some(b"7\n"),
        Some(b"0\n"),
    );
}

#[test]
fn day5_part2_position_less() {
    testprogram_io(
        vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
        None,
        Some(b"8"),
        Some(b"0\n"),
    );
    testprogram_io(
        vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
        None,
        Some(b"7"),
        Some(b"1\n"),
    );
    testprogram_io(
        vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
        None,
        Some(b"10"),
        Some(b"0\n"),
    );
}

#[test]
fn day5_part2_immediate_equal() {
    testprogram_io(
        vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
        None,
        Some(b"8"),
        Some(b"1\n"),
    );
    testprogram_io(
        vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
        None,
        Some(b"100"),
        Some(b"0\n"),
    );
}

#[test]
fn day5_part2_immediate_less() {
    testprogram_io(
        vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
        None,
        Some(b"7"),
        Some(b"1\n"),
    );
    testprogram_io(
        vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
        None,
        Some(b"8"),
        Some(b"0\n"),
    );
}

#[test]
fn day5_part2_position_jump() {
    testprogram_io(
        vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        None,
        Some(b"0"),
        Some(b"0\n"),
    );
    testprogram_io(
        vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        None,
        Some(b"2"),
        Some(b"1\n"),
    );
}

#[test]
fn day5_part2_immediate_jump() {
    testprogram_io(
        vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        None,
        Some(b"0"),
        Some(b"0\n"),
    );
    testprogram_io(
        vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        None,
        Some(b"2"),
        Some(b"1\n"),
    );
}

#[test]
fn day5_large_example() {
    testprogram_io(
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ],
        None,
        Some(b"7"),
        Some(b"999\n"),
    );
    testprogram_io(
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ],
        None,
        Some(b"8"),
        Some(b"1000\n"),
    );
    testprogram_io(
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ],
        None,
        Some(b"9"),
        Some(b"1001\n"),
    );
}
