use rs_quant::quantitative::returns::{expected_returns, ReturnsError};

#[tokio::test]
async fn expected_return() {
    // test error case
    let e: Vec<(String, f64)> = vec![];
    assert!(matches!(
        expected_returns(&e),
        Err(ReturnsError::EmptyInput)
    ));

    let inv = vec![("invalid".to_string(), 1.0)];
    assert!(matches!(
        expected_returns(&inv),
        Err(ReturnsError::LengthError)
    ));

    // test method
    let v = vec![
        ("13-02-2024".to_string(), 0.05),
        ("14-02-2024".to_string(), 0.07),
        ("15-02-2024".to_string(), 0.06),
    ];
    assert!(expected_returns(&v).is_ok());
    assert_eq!(expected_returns(&v).unwrap(), 0.06);
}
