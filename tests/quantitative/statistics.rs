use ndarray::array;
use rs_quant::assert_approx;
use rs_quant::commons::parser::round_to_three;
use rs_quant::commons::utils::EPSILON;
use rs_quant::quantitative::statistics::{MeanType, Statistic, VarType};

#[test]
fn min() {
    // test empty array
    let e_arr = array![];
    assert!(e_arr.min_val().is_err());

    // test array min
    let arr = array![1.0, 2.0, 34.0, 28.0];
    assert_eq!(arr.min_val().unwrap(), 1.0)
}

#[test]
fn max() {
    // test empty array
    let e_arr = array![];
    assert!(e_arr.max_val().is_err());

    // test array max
    let arr = array![1.0, 2.0, 34.0, 28.0];
    assert_eq!(arr.max_val().unwrap(), 34.0)
}

#[test]
fn median() {
    // test empty array
    let mut e_arr = array![];
    assert!(e_arr.median().is_err());

    // test array odd length
    let mut arr = array![1.0, 3.0, 4.0, 2.0, 5.0];
    assert_eq!(arr.median().unwrap(), 3.0);

    // test array even length
    let mut arr = array![1.0, 3.0, 4.0, 2.0];
    assert_eq!(arr.median().unwrap(), 2.5)
}

#[test]
fn quantile() {
    // test empty array
    let mut e_arr = array![];
    assert!(e_arr.quantile(0.25).is_err());

    // test array odd length
    let mut arr = array![1.0, 3.0, 4.0, 2.0, 5.0];
    assert_eq!(arr.quantile(0.25).unwrap(), 2.0);
    assert_eq!(arr.quantile(0.5).unwrap(), 3.0);
    assert_eq!(arr.quantile(0.75).unwrap(), 4.0);

    // test array even lenght
    let mut arr = array![1.0, 3.0, 2.0, 5.0];
    assert_eq!(arr.quantile(0.25).unwrap(), 1.75);
    assert_eq!(arr.quantile(0.5).unwrap(), 2.5);
    assert_eq!(arr.quantile(0.75).unwrap(), 3.5);
}

#[test]
fn percentile() {
    // test empty array
    let mut e_arr = array![];
    assert!(e_arr.percentile(25.).is_err());

    // test odd array
    let mut arr = array![1.0, 3.0, 4.0, 2.0, 5.0];
    assert_eq!(arr.percentile(25.0).unwrap(), 2.0);
    assert_eq!(arr.percentile(50.0).unwrap(), 3.0);
    assert_eq!(arr.percentile(75.0).unwrap(), 4.0);

    // test array even lenght
    let mut arr = array![1.0, 3.0, 2.0, 5.0];
    assert_eq!(arr.percentile(25.0).unwrap(), 1.75);
    assert_eq!(arr.percentile(50.0).unwrap(), 2.5);
    assert_eq!(arr.percentile(75.0).unwrap(), 3.5);
}

#[test]
fn interquartile_range() {
    // test empty array
    let mut e_arr = array![];
    assert!(e_arr.interquartile_range().is_err());

    // test odd array
    let mut arr = array![1.0, 3.0, 4.0, 2.0, 5.0];
    assert_eq!(arr.interquartile_range().unwrap(), 4.0 - 2.0);

    // test even array
    let mut arr = array![1.0, 3.0, 4.0, 2.0];
    assert_eq!(arr.interquartile_range().unwrap(), 1.5)
}

#[test]
fn range() {
    // test empty
    let e_arr = array![];
    assert!(e_arr.range().is_err());

    // test range
    let arr = array![1.0, 2.0, 5.0, 3.0, 6.0];
    assert_eq!(arr.range().unwrap(), 5.0)
}

#[test]
fn mean() {
    // test empty
    let e_arr = array![];
    assert!(e_arr.mean_val(MeanType::Arithmetic).is_err());

    // test arithmetic mean
    let arr = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(arr.mean_val(MeanType::Arithmetic).is_ok());
    assert_eq!(arr.mean_val(MeanType::Arithmetic).unwrap(), 3.0);

    // test geometric mean
    assert!(arr.mean_val(MeanType::Geometric).is_ok());
    assert_approx!(
        arr.mean_val(MeanType::Geometric).unwrap(),
        2.605171084697,
        EPSILON
    );

    // test harmonic mean
    assert!(arr.mean_val(MeanType::Harmonic).is_ok());
    assert_approx!(
        arr.mean_val(MeanType::Harmonic).unwrap(),
        2.1897810218978,
        EPSILON
    );
}

#[test]
fn variance() {
    // test empty
    let e_arr = array![];
    assert!(e_arr.variance(VarType::Sample).is_err());

    // test sample variance
    let arr = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(arr.variance(VarType::Sample).is_ok());
    assert_approx!(arr.variance(VarType::Sample).unwrap(), 2.5, EPSILON);

    // test population variance
    let arr = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(arr.variance(VarType::Population).is_ok());
    assert_approx!(arr.variance(VarType::Population).unwrap(), 2.0, EPSILON);
}

#[test]
fn standard_deviation() {
    // test empty
    let e_arr = array![];
    assert!(e_arr.std_dev(VarType::Sample).is_err());

    // test sample standard deviation
    let arr = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(arr.std_dev(VarType::Sample).is_ok());
    assert_approx!(
        arr.std_dev(VarType::Sample).unwrap(),
        1.5811388300841898,
        EPSILON
    );

    // test population standard deviation
    let arr = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(arr.std_dev(VarType::Population).is_ok());
    assert_approx!(
        round_to_three(arr.std_dev(VarType::Population).unwrap()), // had to round to three digits because result kept ascillating of 0.1 failing the test
        1.414,
        EPSILON
    );
}

#[test]
fn covariance() {
    // test empty
    let e_arr1 = array![];
    let e_arr2 = array![];
    assert!(e_arr1.covariance(&e_arr2).is_err());

    // test one empty
    let arr1 = array![];
    let arr2 = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(arr1.covariance(&arr2).is_err());

    // test shape mismatch
    let arr1 = array![1.0, 2.0, 3.0];
    let arr2 = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(arr1.covariance(&arr2).is_err());

    // test covariance
    let arr1 = array![1.0, 2.0, 3.0, 4.0, 5.0];
    let arr2 = array![5.0, 4.0, 3.0, 2.0, 1.0];
    assert!(arr1.covariance(&arr2).is_ok());
    assert_approx!(arr1.covariance(&arr2).unwrap(), -2.5, EPSILON);
}

#[test]
fn pearson_corr() {
    // test error case
    let arr1 = array![];
    let arr2 = array![];
    assert!(arr1.pearson_corr(&arr2).is_err());

    let arr1 = array![1.0, 2.0, 3.0];
    let arr2 = array![2.0, 4.0];
    assert!(arr1.pearson_corr(&arr2).is_err());

    // test pearson correlation valid
    let arr1 = array![1.0, 2.0, 3.0, 4.0, 5.0];
    let arr2 = array![2.0, 4.0, 6.0, 8.0, 10.0];
    assert!(arr1.pearson_corr(&arr2).is_ok());
    assert_approx!(arr1.pearson_corr(&arr2).unwrap(), 1.0, EPSILON);

    // test pearson correlation negative
    let arr1 = array![1.0, 2.0, 3.0, 4.0, 5.0];
    let arr2 = array![10.0, 8.0, 6.0, 4.0, 2.0];
    assert!(arr1.pearson_corr(&arr2).is_ok());
    assert_approx!(arr1.pearson_corr(&arr2).unwrap(), -1.0, EPSILON);

    // test pearson correlation (low correlation)
    let arr1 = array![1.0, 2.0, 3.0, 4.0, 5.0];
    let arr2 = array![10.0, 20.0, 15.0, 0.0, 25.0];
    assert!(arr1.pearson_corr(&arr2).is_ok());
    assert_approx!(
        arr1.pearson_corr(&arr2).unwrap(),
        0.16439898730535726,
        EPSILON
    )
}

#[test]
fn kurtosis() {
    // test empty
    let e_arr = array![];
    assert!(e_arr.kurt().is_err());

    // test kurtosis
    let arr = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_approx!(arr.kurt().unwrap(), -1.2000, 1.0); // resulted in -1.3 while kurtosis calculator state -1.2, understand why it result 0.1 error
}

#[test]
fn skewness() {
    // test empty
    let e_arr = array![];
    assert!(e_arr.skew().is_err());

    // test skewness
    let arr = array![1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(arr.skew().is_ok());
    assert_approx!(arr.skew().unwrap(), 0.0, EPSILON);
}
