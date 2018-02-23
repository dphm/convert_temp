pub fn to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
