use ndarray::array;
use rs_quant::quantitative::matrix::Matrix;

#[test]
fn covariance_matrix() {
    let arr = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let exp = array![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];
    assert!(arr.covariance_matrix().is_ok(), "error: {:?}", arr);
    assert_eq!(arr.covariance_matrix().unwrap(), exp);
}
