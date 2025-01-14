use rs_quant::assert_proxy;
use rs_quant::quantitative::math::statistics::{
    MeanType, StandardDeviationType, Statistics, StatisticsError, VarianceType,
};
use rs_quant::utils::common::EPSILON as prox_err;

#[test]
fn test_min() {
    // test empty vector
    let empty: Vec<f64> = vec![];
    assert!(matches!(empty.min(), Err(StatisticsError::EmptyVector)));

    // test valid one element vector
    let vec = vec![50.0];
    assert!(vec.min().is_ok());
    assert_eq!(vec.min().unwrap(), 50.0);

    // test valid vector
    let vec: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    assert_eq!(vec.min().unwrap(), 1.0);
}

#[test]
fn test_max() {
    // test empty vector
    let empty: Vec<f64> = vec![];
    assert!(matches!(empty.max(), Err(StatisticsError::EmptyVector)));

    // test valid one element vector
    let vec = vec![50.0];
    assert!(vec.max().is_ok());
    assert_eq!(vec.max().unwrap(), 50.0);

    // test valid vector
    let vec: Vec<f64> = vec![1.0, 2.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    assert_eq!(vec.max().unwrap(), 9.0);
}

#[test]
fn test_median() {
    // test empty vector
    let empty: Vec<f64> = vec![];
    assert!(matches!(empty.median(), Err(StatisticsError::EmptyVector)));

    // test odd length vector
    let odd = vec![3.0, 1.0, 2.0];
    assert_eq!(odd.median().unwrap(), 2.0);

    // test even length vector
    let even = vec![3.0, 1.0, 4.0, 2.0];
    assert_eq!(even.median().unwrap(), 2.5);
}

#[test]
fn test_percentile() {
    // test empty vector
    let empty: Vec<f64> = vec![];
    assert!(matches!(
        empty.percentile(0.5),
        Err(StatisticsError::EmptyVector)
    ));

    // test invalid percentile
    let vec = vec![1.0, 2.0, 3.0];
    assert!(matches!(
        vec.percentile(1.5),
        Err(StatisticsError::PercentileError)
    ));

    // test valid percentile
    let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(vec.percentile(0.25).unwrap(), 2.0); // first quartile
    assert_eq!(vec.percentile(0.5).unwrap(), 3.0); // median
    assert_eq!(vec.percentile(0.75).unwrap(), 4.0); // third quartile
}

#[test]
fn test_quantile() {
    // test empty vector
    let empty: Vec<f64> = vec![];
    assert!(matches!(
        empty.quantile(0.5),
        Err(StatisticsError::EmptyVector)
    ));

    // test invalid percentile
    let vec = vec![1.0, 2.0, 3.0];
    assert!(matches!(
        vec.quantile(1.5),
        Err(StatisticsError::QuantileError)
    ));

    // test valid percentile
    let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(vec.quantile(0.25).unwrap(), 2.0); // first quartile
    assert_eq!(vec.quantile(0.5).unwrap(), 3.0); // median
    assert_eq!(vec.quantile(0.75).unwrap(), 4.0); // third quartile
}

#[test]
fn test_interquantile_range() {
    // test empty vector
    let empty: Vec<f64> = vec![];
    assert!(matches!(
        empty.interquantile_range(),
        Err(StatisticsError::EmptyVector)
    ));

    // test valid vector
    let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    assert_eq!(vec.interquantile_range().unwrap(), 4.0); // Q3 - Q6 = 6.0 - 2.0
}

#[test]
fn test_range() {
    // test empty vector
    let empty = vec![];
    assert!(matches!(empty.range(), Err(StatisticsError::EmptyVector)));

    // test valid vector
    let vec = vec![1.0, 5.0, 3.0, 7.0, 2.0];
    assert_eq!(vec.range().unwrap(), 6.0); // max - min = 7.0 - 1.0
}

#[test]
fn test_mean() {
    // test empty vector
    let empty: Vec<f64> = vec![];
    assert!(matches!(
        empty.mean(MeanType::Arithmetic),
        Err(StatisticsError::EmptyVector)
    ));

    // test arithmetic mean
    let vec = vec![1.0, 5.0, 6.0, 2.0];
    assert_proxy!(vec.mean(MeanType::Arithmetic).unwrap(), 3.5, prox_err);

    // test harmonic mean
    assert_proxy!(
        vec.mean(MeanType::Harmonic).unwrap(),
        2.1428571428571,
        prox_err
    );

    // test geometric mean
    assert_proxy!(
        vec.mean(MeanType::Geometric).unwrap(),
        2.783157683714,
        prox_err
    );

    // test quadratic mean
    assert_proxy!(
        vec.mean(MeanType::Quadratic).unwrap(),
        4.06201920231798,
        prox_err
    );

    // test cubic mean
    assert_proxy!(
        vec.mean(MeanType::Cubic).unwrap(),
        4.439520008713004,
        prox_err
    );
}

#[test]
fn test_variance() {
    // test empty vector
    let empty: Vec<f64> = vec![];
    assert!(matches!(
        empty.variance(VarianceType::CorrectedSample),
        Err(StatisticsError::EmptyVector)
    ));

    // test valid vector corrected sample variance
    let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_proxy!(
        vec.variance(VarianceType::CorrectedSample).unwrap(),
        2.5,
        prox_err
    );

    // test valid vector uncorrected sample variance
    assert_proxy!(
        vec.variance(VarianceType::UncorrectedSample).unwrap(),
        2.0,
        prox_err
    );
}

#[test]
fn test_standard_deviation() {
    // test inconsistent vector length
    let vec = vec![1.0];
    assert!(matches!(
        vec.standard_deviation(StandardDeviationType::CorrectedSample),
        Err(StatisticsError::VectorLengthLessThanTwo)
    ));

    // test corrected sample standard deviation
    let vec = vec![1.0, 2.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    assert_proxy!(
        vec.standard_deviation(StandardDeviationType::CorrectedSample)
            .unwrap(),
        2.751622897751175,
        prox_err
    );

    // test uncorrected sample standard deviation
    assert_proxy!(
        vec.standard_deviation(StandardDeviationType::UncorrectedSample)
            .unwrap(),
        2.5475077857324,
        prox_err
    );
}

#[test]
fn test_covariance() {
    // test invalid vector: length not equal
    let x = vec![1.0, 2.0];
    let y = vec![1.0];
    assert!(matches!(
        x.covariance(&y),
        Err(StatisticsError::VectorLengthNotEqual)
    ));

    // test invalid vector: vector length less than two
    let x = vec![1.0];
    let y = vec![1.0];
    assert!(matches!(
        x.covariance(&y),
        Err(StatisticsError::VectorLengthLessThanTwo)
    ));

    // test valid vector
    let x = vec![2.1, 2.5, 4.0, 3.6];
    let y = vec![8.0, 10.0, 12.0, 14.0];
    assert_proxy!(x.covariance(&y).unwrap(), 2.0, prox_err);
}

#[test]
fn test_correlation() {
    // test std zero error
    let x = vec![1.0, 1.0];
    let y = vec![1.0, 2.0];
    assert!(matches!(
        x.correlation(&y),
        Err(StatisticsError::StandardDeviationZero)
    ));

    // test valid vector
    let x = vec![2.1, 2.5, 4.0, 3.6];
    let y = vec![8.0, 10.0, 12.0, 14.0];
    assert_proxy!(x.correlation(&y).unwrap(), 0.8642268028735162, prox_err);
}

#[test]
fn test_skewness() {
    // test invalid vector: vector length less than three
    let vec = vec![1.0];
    assert!(matches!(
        vec.skewness(),
        Err(StatisticsError::VectorLengthLessThanThree)
    ));

    // test valid vector
    let vec = vec![1.0, 2.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    assert_proxy!(vec.skewness().unwrap(), 0.352308329, prox_err);
}

#[test]
fn test_kurtosis() {
    // test invalid vector: vector length less than four
    let vec = vec![1.0];
    assert!(matches!(
        vec.kurtosis(),
        Err(StatisticsError::VectorLengthLessThanFour)
    ));

    // test valid vector
    let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    assert_proxy!(vec.kurtosis().unwrap(), -0.207812500, prox_err);
}
