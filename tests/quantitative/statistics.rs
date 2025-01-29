use ndarray::Array1;
use rs_quant::quantitative::statistics::Stat;

#[test]
fn min() {
    // test empty array
    let e_vec: Vec<f64> = vec![];
    let e_arr: Array1<f64> = Array1::from_vec(e_vec);
    assert!(e_arr.min().is_err());

    // test array min
    let v = vec![1.0, 2.0, 34.0, 28.0];
    let arr = Array1::from_vec(v);
    assert_eq!(arr.min().unwrap(), 1.0)
}

#[test]
fn max() {
    // test empty arr
    let e_vec: Vec<f64> = vec![];
    let e_arr = Array1::from_vec(e_vec);
    assert!(e_arr.max().is_err());

    // test array max
    let v = vec![1.0, 2.0, 34.0, 28.0];
    let arr = Array1::from_vec(v);
    assert_eq!(arr.max().unwrap(), 34.0)
}
