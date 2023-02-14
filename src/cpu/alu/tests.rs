use super::*;

// This macro calls a given arithmetic function (e.g., add8) on the two operands and then assert the result and the
// expected flag values.
macro_rules! assert_arithmetic {
    ($f:ident, $flags:ident, $x:expr, $y:expr, $expected:expr, $z_flag:expr, $n_flag:expr, $h_flag:expr, $c_flag:expr) => {
        let result = $f(&mut $flags, $x, $y);

        assert_eq!(
            result,
            $expected,
            "{}({}, {}) gave {} but expected {}",
            stringify!($f),
            $x,
            $y,
            result,
            $expected,
        );

        assert_eq!(
            $flags.get(Flag::Zero),
            $z_flag,
            "{}({}, {}) did not set zero flag correctly",
            stringify!($f),
            $x,
            $y
        );
        assert_eq!(
            $flags.get(Flag::Subtraction),
            $n_flag,
            "{}({}, {}) did not set subtraction flag correctly",
            stringify!($f),
            $x,
            $y
        );
        assert_eq!(
            $flags.get(Flag::HalfCarry),
            $h_flag,
            "{}({}, {}) did not set half carry flag correctly",
            stringify!($f),
            $x,
            $y
        );
        assert_eq!(
            $flags.get(Flag::Carry),
            $c_flag,
            "{}({}, {}) did not set carry flag correctly",
            stringify!($f),
            $x,
            $y
        );
    };
}

macro_rules! assert_arithmetic_8_bit {
        ($func:ident, $carry_func:ident, $n_flag:literal, $tests:expr) => {
            let mut flags = Flags::default();

            for (x, y, expected, h_flag, c_flag) in $tests {
                let z_flag = expected == 0;
                assert_arithmetic!($func, flags, x, y, expected, z_flag, $n_flag, h_flag, c_flag);

                // test adc8, sbc8, etc. with carry flag reset (i.e., ensure they function the same as add8, sub8, etc.
                // respectively)
                flags.set(Flag::Carry, false);
                assert_arithmetic!($carry_func, flags, x, y, expected, z_flag, $n_flag, h_flag, c_flag);
            }
        };
    }

macro_rules! assert_arithmetic_8_bit_carry {
    ($func:ident, $n_flag:literal, $tests:expr) => {
        let mut flags = Flags::default();

        for (x, y, expected, h_flag, c_flag) in $tests {
            let z = expected == 0;
            flags.set(Flag::Carry, true);
            assert_arithmetic!($func, flags, x, y, expected, z, $n_flag, h_flag, c_flag);
        }
    };
}

macro_rules! assert_bitwise {
    ($func:ident, $x:expr, $y:expr, $expected:expr, $h_flag:literal) => {
        let mut flags = Flags::default();

        // set flags that need to be reset to ensure that they indeed are reset
        flags
            .set(Flag::Subtraction, false)
            .set(Flag::HalfCarry, false)
            .set(Flag::Carry, false);

        let result = $func(&mut flags, $x, $y);
        assert_eq!(
            result,
            $expected,
            "{}({}, {}) gave {:b} but expected {:b}",
            stringify!($func),
            $x,
            $y,
            result,
            $expected
        );

        let mut expected_flags = Flags::default();
        expected_flags.set(Flag::Zero, result == 0);
        expected_flags.set(Flag::HalfCarry, $h_flag);

        assert_eq!(flags, expected_flags);
    };
}

#[test]
fn addition_8_bit() {
    assert_arithmetic_8_bit!(
        add8,
        adc8,
        false, // addition should always reset the subtraction flag
        [
            (0, 0, 0, false, false),
            (15, 1, 16, true, false),
            (255, 1, 0, true, true),
            (200, 200, 144, true, true),
        ]
    );
}

#[test]
fn addition_8_bit_carry() {
    assert_arithmetic_8_bit_carry!(
        adc8,
        false, // subtraction flag
        [
            (0, 0, 1, false, false),
            (14, 1, 16, true, false),
            (200, 55, 0, true, true),
        ]
    );
}

#[test]
fn subtraction() {
    assert_arithmetic_8_bit!(
        sub8,
        sbc8,
        true, // subtraction should always set the subtraction flag
        [
            (0, 0, 0, false, false),
            (100, 10, 90, true, false),
            (50, 100, 206, true, true),
        ]
    );
}

#[test]
fn subtraction_carry() {
    assert_arithmetic_8_bit_carry!(
        sbc8,
        true, // subtraction flag
        [(1, 0, 0, false, false),]
    );
}

#[test]
fn addition_16_bit() {
    let tests = [
        (0, 0, 0, false, false),
        (200, 300, 500, false, false),
        (4095, 5, 4100, true, false),
        (30500, 60500, 25464, true, true),
    ];

    let mut flags = Flags::default();
    let n = false;

    // we expect add16 to not change the zero flag even if the result is zero, so we set the flag now and ensure it
    // remains set in each test case
    let z = true;
    flags.set(Flag::Zero, z);

    for (x, y, expected, h, c) in tests {
        assert_arithmetic!(add16, flags, x, y, expected, z, n, h, c);
    }
}

#[test]
fn bitwise_operations() {
    assert_bitwise!(bitwise_and, 0, 0, 0, true);
    assert_bitwise!(bitwise_and, 0b1001010, 0b1000101, 0b1000000, true);
    assert_bitwise!(bitwise_or, 0b1010, 0b100110, 0b101110, false);
    assert_bitwise!(bitwise_xor, 0b11111, 0b10100, 0b01011, false);
}

#[test]
fn complement() {
    let mut flags = Flags::default();
    flags.set(Flag::Zero, true).set(Flag::Carry, true); // set to ensure they're unaffected

    assert_eq!(bitwise_not(&mut flags, 0b101010), 0b11010101);

    let mut expected_flags = Flags::default();
    expected_flags
        .set(Flag::Zero, true)
        .set(Flag::Subtraction, true)
        .set(Flag::HalfCarry, true)
        .set(Flag::Carry, true);

    assert_eq!(flags, expected_flags);
}

#[test]
fn rotation() {
    // TODO
}

#[test]
fn bit_testing() {
    let tests = [
        (0, 0, true),
        (0, 0xFF, false),
        (3, 0b1110111, true),
        (7, 0b10001000, false),
    ];

    let mut flags = Flags::default();

    for (bit, value, expected) in tests {
        test_bit(&mut flags, bit, value);

        assert_eq!(
            expected,
            flags.get(Flag::Zero),
            "bit {bit} of value {value} tested, expected {expected} zero flag"
        );
        assert!(!flags.get(Flag::Subtraction));
        assert!(!flags.get(Flag::HalfCarry));
    }
}

#[test]
fn bit_setting() {
    assert_eq!(set_bit(0, 0), 1);
    assert_eq!(set_bit(0, 1), 1);
    assert_eq!(set_bit(2, 0b10011), 0b10111);
    assert_eq!(set_bit(7, 0b01111111), 0xFF);
}

#[test]
fn bit_resetting() {
    assert_eq!(reset_bit(0, 0), 0);
    assert_eq!(reset_bit(0, 1), 0);
    assert_eq!(reset_bit(3, 0b1010), 0b10);
    assert_eq!(reset_bit(7, 0b10000000), 0);
}

#[test]
fn nibble_swapping() {
    let mut flags = Flags::default();
    // we need to ensure flags are reset so let's set them now
    flags
        .set(Flag::Subtraction, true)
        .set(Flag::HalfCarry, true)
        .set(Flag::Carry, true);

    assert_eq!(swap_nibbles(&mut flags, 0xAB), 0xBA);
    assert!(!flags.get(Flag::Zero));
    assert!(!flags.get(Flag::Subtraction));
    assert!(!flags.get(Flag::HalfCarry));
    assert!(!flags.get(Flag::Carry));

    assert_eq!(swap_nibbles(&mut flags, 0), 0);
    assert!(flags.get(Flag::Zero));
}
