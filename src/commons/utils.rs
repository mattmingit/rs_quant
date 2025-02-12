// Epsilon constant for use in testing
// It is used to compare floating point numbers
// It is set to f64::sqrt(f64::EPSILON) which is the square root of the smallest positive number representable by f64
pub const EPSILON: f64 = 0.000_000_014_901_161_193_847_656;

// macro for testing floating point numbers
#[macro_export]
macro_rules! assert_approx {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            ($x - $y <= $d) && ($y - $x <= $d),
            "\nLeft: \t\t{}, \nRigth: \t\t{}, \nPrecision: \t\t{}",
            $x,
            $y,
            $d
        )
    };
}
