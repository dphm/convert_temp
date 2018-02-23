#[macro_export]
macro_rules! assert_eq_within {
    ($a: expr, $b: expr, $delta: expr) => (
        assert!(
            ($a - $b).abs() < $delta,
            "{} is not approximately equal to {}", $a, $b
        );
    )
}
