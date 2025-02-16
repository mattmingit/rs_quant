// helper function to round f64 to three digits
pub fn round_to_three(n: f64) -> f64 {
    (n * 1000.0).round() / 1000.0
}

// helper function to round f64 to four digits
pub fn round_to_four(n: f64) -> f64 {
    (n * 10000.0).round() / 10000.0
}
