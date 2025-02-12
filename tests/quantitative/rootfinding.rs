use rs_quant::quantitative::rootfinding::{
    BrentsMethod, NewtonRaphsonMethod, RootFinding, RootFindingError, SecantMethod,
};

#[test]
fn newton_raphson_success() {
    let method = NewtonRaphsonMethod { x0: 1.0 };

    // define f(x) = X^2 - 4, f'(x) = 2x
    let f = |x: f64| (x * x - 4.0, 2.0 * x);
    let root = method.find_root(f, 1e-6, 100);
    assert!(root.is_ok());
    let root = root.unwrap();
    assert!((root - 2.0).abs() < 1e-6 || (root + 2.0).abs() < 1e-6)
}

#[test]
fn test_newton_raphson_zero_derivative() {
    let method = NewtonRaphsonMethod { x0: 0.0 };

    // Define f(x) = x^2, f'(x) = 2x (zero derivative at x = 0)
    let f = |x: f64| (x * x, 2.0 * x);

    let root = method.find_root(f, 1e-6, 1000);
    if root.is_ok() {
        println!("unexpected result: {:?}", root);
    }
    assert!(matches!(root, Err(RootFindingError::ZeroDerivative))); // Should return error due to zero derivative
}

#[test]
fn test_secant_method_success() {
    let method = SecantMethod { x0: 1.0, x1: 2.0 };

    let f = |x: f64| x * x - 4.0; // Root at ±2

    let root = method.find_root(f, 1e-6, 100);
    assert!(root.is_ok());
    let root = root.unwrap();
    assert!((root - 2.0).abs() < 1e-6 || (root + 2.0).abs() < 1e-6);
}

#[test]
fn test_secant_method_no_convergence() {
    let method = SecantMethod {
        x0: 1.0,
        x1: 1.0000001,
    };

    let f = |x: f64| x * x - 4.0;

    let root = method.find_root(f, 1e-6, 5);
    assert!(matches!(root, Err(RootFindingError::NoConvergence)));
}

#[test]
fn test_brents_method_success() {
    let method = BrentsMethod { a: 1.0, b: 3.0 };

    let f = |x: f64| x * x - 4.0; // Root at ±2

    let root = method.find_root(f, 1e-6, 100);
    assert!(root.is_ok());
    let root = root.unwrap();
    assert!((root - 2.0).abs() < 1e-6);
}

#[test]
fn test_brents_method_invalid_interval() {
    let method = BrentsMethod { a: 3.0, b: 5.0 };

    let f = |x: f64| x * x - 4.0;

    let root = method.find_root(f, 1e-6, 100);
    assert!(matches!(root, Err(RootFindingError::InvalidInterval)));
}
