use ndarray::{array, Array1};
use rs_quant::quantitative::sequence::Sequence;

#[test]
fn seq() {
    // test error case
    assert!(Array1::<f64>::seq(1.0, 5.0, 0.0).is_err());

    // test sequence
    assert_eq!(
        Array1::<f64>::seq(1.0, 5.0, 1.0).unwrap(),
        array![1.0, 2.0, 3.0, 4.0, 5.0]
    )
}

#[test]
fn rep() {
    // test repetition
    assert_eq!(
        Array1::<f64>::rep(2.0, 4).unwrap(),
        array![2.0, 2.0, 2.0, 2.0]
    );

    assert_eq!(Array1::<f64>::rep(2.0, 0).unwrap().len(), 0);
}

#[test]
fn linspace() {
    // test error case
    assert!(Array1::<f64>::lin_space(5.0, 5.0, 5).is_err());
    assert!(Array1::<f64>::lin_space(0.0, 10.0, 0).is_err());

    // test linearly spaced sequence
    assert_eq!(
        Array1::<f64>::lin_space(0.0, 10.0, 5).unwrap(),
        array![0.0, 2.5, 5.0, 7.5, 10.0]
    );
}

#[test]
fn log_space() {
    // test error case
    assert!(Array1::<f64>::log_space(5.0, 5.0, 5).is_err());
    assert!(Array1::<f64>::log_space(0.0, 10.0, 0).is_err());

    // test logarithmically spaced sequence
    assert_eq!(
        Array1::<f64>::log_space(0.0, 2.0, 3).unwrap(),
        array![1.0, 10.0, 100.0]
    );
}

#[test]
fn cumsum() {
    // test error case
    assert!(Array1::<f64>::cumsum(&array![]).is_err());

    // test cumulative sum sequence
    assert_eq!(
        Array1::<f64>::cumsum(&array![1.0, 2.0, 3.0, 4.0]).unwrap(),
        array![1.0, 3.0, 6.0, 10.0]
    );
    assert_eq!(
        Array1::<f64>::cumsum(&Array1::<f64>::zeros(3)).unwrap(),
        array![0.0, 0.0, 0.0]
    )
}
