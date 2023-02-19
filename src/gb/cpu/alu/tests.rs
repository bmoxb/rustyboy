use super::*;

// This macro calls a given arithmetic function (e.g., add8) on the two operands and then assert the result and the
// expected flag values.
macro_rules! assert_arithmetic {
    ($f:ident, $flags:ident, $x:expr, $y:expr, $expected:expr, $c_flag:expr, $h_flag:expr, $n_flag:expr, $z_flag:expr) => {
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
            $flags,
            Flags::new($c_flag, $h_flag, $n_flag, $z_flag),
            "{}({}, {}) did not set flags correctly",
            stringify!($f),
            $x,
            $y
        );
    };
}

macro_rules! assert_bitwise {
    ($func:ident, $x:expr, $y:expr, $expected:expr, $h_flag:literal) => {
        let mut flags = Flags::new(true, true, true, false); // set flags that need to be reset to ensure that they are

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

        assert_eq!(flags, Flags::new(false, $h_flag, false, result == 0));
    };
}

#[test]
fn addition_8_bit() {
    let mut flags = Flags::default();

    assert_arithmetic!(add8, flags, 0, 0, 0, false, false, false, true);
    assert_arithmetic!(add8, flags, 15, 1, 16, false, true, false, false);
    assert_arithmetic!(add8, flags, 255, 1, 0, true, true, false, true);
    assert_arithmetic!(add8, flags, 200, 200, 144, true, true, false, false);

    flags.set_carry(false);
    assert_arithmetic!(adc8, flags, 15, 1, 16, false, true, false, false);
    assert_arithmetic!(adc8, flags, 200, 200, 144, true, true, false, false);
}

#[test]
fn addition_8_bit_carry() {
    let mut flags = Flags::default();

    flags.set_carry(true);
    assert_arithmetic!(adc8, flags, 0, 0, 1, false, false, false, false);
    flags.set_carry(true);
    assert_arithmetic!(adc8, flags, 14, 1, 16, false, true, false, false);
    flags.set_carry(true);
    assert_arithmetic!(adc8, flags, 200, 55, 0, true, true, false, true);
}

#[test]
fn subtraction() {
    let mut flags = Flags::default();

    assert_arithmetic!(sub8, flags, 0, 0, 0, false, false, true, true);
    assert_arithmetic!(sub8, flags, 100, 10, 90, false, true, true, false);
    assert_arithmetic!(sub8, flags, 50, 100, 206, true, true, true, false);

    flags.set_carry(false);
    assert_arithmetic!(sbc8, flags, 0, 0, 0, false, false, true, true);
    assert_arithmetic!(sbc8, flags, 50, 100, 206, true, true, true, false);
}

#[test]
fn subtraction_carry() {
    let mut flags = Flags::default();
    flags.set_carry(true);
    assert_arithmetic!(sbc8, flags, 1, 0, 0, false, false, true, true);
}

#[test]
fn addition_16_bit() {
    let mut flags = Flags::default();

    // we expect add16 to not change the zero flag even if the result is zero, so we set the flag now and ensure it
    // remains set
    flags.set_zero(true);

    assert_arithmetic!(add16, flags, 0, 0, 0, false, false, false, true);
    assert_arithmetic!(add16, flags, 200, 300, 500, false, false, false, true);
    assert_arithmetic!(add16, flags, 4095, 5, 4100, false, true, false, true);
    assert_arithmetic!(add16, flags, 30500, 60500, 25464, true, true, false, true);
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
    let mut flags = Flags::new(true, false, false, true); // set zero and carry to ensure they're unaffected
    assert_eq!(bitwise_not(&mut flags, 0b101010), 0b11010101);
    assert_eq!(flags, Flags::new(true, true, true, true));
}

#[test]
fn nibble_swapping() {
    let mut flags = Flags::new(true, true, true, false); // set carry, half carry, subtraction to ensure they're cleared

    assert_eq!(swap_nibbles(&mut flags, 0xAB), 0xBA);
    assert_eq!(flags, Flags::default()); // ensure all cleared

    assert_eq!(swap_nibbles(&mut flags, 0), 0);
    assert!(flags.zero());
}

#[test]
fn rotation() {
    let mut flags = Flags::default();

    assert_eq!(rotate_left(&mut flags, 0b11001100), 0b10011001);
    assert_eq!(flags, Flags::new(true, false, false, false));
}
