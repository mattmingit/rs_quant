use rs_quant::{
    assert_proxy,
    quantitative::{
        math::{matrices::StatisticsMatrices, statistics::StatisticsError},
        returns::Return,
    },
    utils::common::EPSILON as prox_err,
};

#[test]
fn test_covariance_matrix() {
    // test empty vector
    let empty: Vec<Vec<f64>> = vec![];
    assert!(matches!(
        empty.covariance_matrix(),
        Err(StatisticsError::EmptyVector)
    ));

    // test invalid vector: length not equal
    let vec = vec![vec![1.0, 2.0], vec![1.0, 2.0, 3.0]];
    assert!(matches!(
        vec.covariance_matrix(),
        Err(StatisticsError::VectorLengthNotEqual)
    ));

    // test invalid vector:: vector less than two
    let vec = vec![vec![1.0], vec![2.0]];
    assert!(matches!(
        vec.covariance_matrix(),
        Err(StatisticsError::VectorLengthLessThanTwo)
    ));

    // test valid vector
    let vec = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];

    let expected = vec![
        vec![1.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
    ];

    assert_eq!(vec.covariance_matrix().unwrap(), expected);

    // test covariance matrix non trivial
    let vec = vec![vec![1.0, 2.0, 3.0], vec![1.0, 3.0, 5.0]];
    let expected = vec![vec![1.0, 2.0], vec![2.0, 4.0]];

    assert_eq!(vec.covariance_matrix().unwrap(), expected);
}

#[test]
fn test_correlation_matrix() {
    // test empty vector
    let empty: Vec<Vec<f64>> = vec![];
    assert!(matches!(
        empty.correlation_matrix(),
        Err(StatisticsError::EmptyVector)
    ));

    // test invalid vector: less than two
    let vec = vec![vec![1.0, 2.0], vec![1.0, 2.0, 3.0]];
    assert!(matches!(
        vec.correlation_matrix(),
        Err(StatisticsError::VectorLengthNotEqual)
    ));

    // test valid vector
    let vec = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];

    let expected = vec![
        vec![1.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
        vec![1.0, 1.0, 1.0],
    ];

    assert_eq!(vec.correlation_matrix().unwrap(), expected);

    // test correlation matrix non trivial
    let vec = vec![vec![1.0, 2.0, 3.0], vec![1.0, 3.0, 5.0]];

    let expected = vec![vec![1.0, 1.0], vec![1.0, 1.0]];

    assert_eq!(vec.correlation_matrix().unwrap(), expected);
}

#[test]
fn test_covariance_and_correlation_matrix_with_return() {
    // Sample data using the Return struct
    let data = [
        vec![
            Return {
                datetime: "2025-01-01".to_string(),
                asset_return: 0.01,
            },
            Return {
                datetime: "2025-01-02".to_string(),
                asset_return: 0.02,
            },
            Return {
                datetime: "2025-01-03".to_string(),
                asset_return: 0.03,
            },
        ],
        vec![
            Return {
                datetime: "2025-01-01".to_string(),
                asset_return: 0.04,
            },
            Return {
                datetime: "2025-01-02".to_string(),
                asset_return: 0.05,
            },
            Return {
                datetime: "2025-01-03".to_string(),
                asset_return: 0.06,
            },
        ],
        vec![
            Return {
                datetime: "2025-01-01".to_string(),
                asset_return: -0.01,
            },
            Return {
                datetime: "2025-01-02".to_string(),
                asset_return: -0.02,
            },
            Return {
                datetime: "2025-01-03".to_string(),
                asset_return: -0.03,
            },
        ],
    ];

    // Extract `asset_return` into Vec<Vec<f64>>
    let matrix: Vec<Vec<f64>> = data
        .iter()
        .map(|row| row.iter().map(|r| r.asset_return).collect())
        .collect();

    // Test covariance matrix
    let covariance_result = matrix.covariance_matrix();
    assert!(
        covariance_result.is_ok(),
        "Covariance matrix calculation failed"
    );
    let covariance_matrix = covariance_result.unwrap();

    // Validate covariance matrix shape
    assert_eq!(
        covariance_matrix.len(),
        matrix.len(),
        "Covariance matrix row count mismatch"
    );
    for row in &covariance_matrix {
        assert_eq!(
            row.len(),
            matrix.len(),
            "Covariance matrix column count mismatch"
        );
    }

    // Test correlation matrix
    let correlation_result = matrix.correlation_matrix();
    assert!(
        correlation_result.is_ok(),
        "Correlation matrix calculation failed"
    );
    let correlation_matrix = correlation_result.unwrap();

    // Validate correlation matrix shape
    assert_eq!(
        correlation_matrix.len(),
        matrix.len(),
        "Correlation matrix row count mismatch"
    );
    for row in &correlation_matrix {
        assert_eq!(
            row.len(),
            matrix.len(),
            "Correlation matrix column count mismatch"
        );
    }

    // Check if diagonal elements of the correlation matrix are 1
    for (i, row) in correlation_matrix.iter().enumerate() {
        assert_proxy!(row[i], 1.0, prox_err);
    }
}
