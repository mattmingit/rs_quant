use rs_quant::assert_proxy;
use rs_quant::quantitative::math::regression::Regression;
use rs_quant::quantitative::math::statistics::StatisticsError;
use rs_quant::utils::common::EPSILON as prox_err;

#[test]
fn test_beta() {
    // test invalid vector: length not equal
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    assert!(matches!(
        x.beta(&y),
        Err(StatisticsError::VectorLengthNotEqual)
    ));

    // test valid vector
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    assert_proxy!(x.beta(&y).unwrap(), 2.0, prox_err);
}
