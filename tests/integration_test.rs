extern crate convert_temp;

#[macro_use]
mod common;

use convert_temp::to_fahrenheit;
use convert_temp::to_celsius;

const DELTA: f64 = 1.0e-7;
const ABSOLUTE_ZERO_F: f64 = -459.67;
const ABSOLUTE_ZERO_C: f64 = -273.15;

macro_rules! test_from_fahrenheit {
    ($test_name: ident, $from: expr, $to: expr) => (
        #[test]
        fn $test_name() {
            let from_f = to_celsius($from);
            assert_eq_within!($to, from_f, DELTA);
        }
    )
}

macro_rules! test_from_celsius {
    ($test_name: ident, $from: expr, $to: expr) => (
        #[test]
        fn $test_name() {
            let from_c = to_fahrenheit($from);
            assert_eq_within!($to, from_c, DELTA);
        }
    )
}

test_from_fahrenheit!(absolute_zero_f, ABSOLUTE_ZERO_F, ABSOLUTE_ZERO_C);
test_from_fahrenheit!(minus_forty_f,        -40.0,   -40.0);
test_from_fahrenheit!(thirty_two_f,          32.0,     0.0);
test_from_fahrenheit!(two_hundred_twelve_f, 212.0,   100.0);

test_from_celsius!(absolute_zero_c, ABSOLUTE_ZERO_C, ABSOLUTE_ZERO_F);
test_from_celsius!(minus_forty_c,        -40.0, -40.0);
test_from_celsius!(thirty_two_c,           0.0,  32.0);
test_from_celsius!(two_hundred_twelve_c, 100.0, 212.0);

#[test]
fn to_fahrenheit_to_celsius() {
    let temps = [-123.45, 0.0, 123.45];

    for &t in temps.iter() {
        assert_eq_within!(t, to_fahrenheit(to_celsius(t)), DELTA);
    }
}

#[test]
fn to_celsius_to_fahrenheit() {
    let temps = [-123.45, 0.0, 123.45];

    for &t in temps.iter() {
        assert_eq_within!(t, to_celsius(to_fahrenheit(t)), DELTA);
    }
}
