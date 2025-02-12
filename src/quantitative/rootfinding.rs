use roots::{find_root_brent, SimpleConvergency};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RootFindingError {
    #[error("Derivative is too small (close to zero), can not continue iteration.")]
    ZeroDerivative, // derivative is to small
    #[error("Max number of iterations reached without convergence.")]
    NoConvergence, // max iterations reached
    #[error("Invalid interval: f(a) and f(b) must have opposite signs.")]
    InvalidInterval, // Brent method require f(a) * f(b) < 0
}

pub trait RootFinding {
    type Output;

    fn find_root<F>(&self, f: F, tol: f64, max_iter: usize) -> Result<f64, RootFindingError>
    where
        F: Fn(f64) -> Self::Output;
}

pub struct NewtonRaphsonMethod {
    pub x0: f64,
}

impl RootFinding for NewtonRaphsonMethod {
    type Output = (f64, f64);

    fn find_root<F>(&self, f: F, tol: f64, max_iter: usize) -> Result<f64, RootFindingError>
    where
        F: Fn(f64) -> (f64, f64),
    {
        let mut x = self.x0;
        let (_fx, dfx) = f(x);

        // avoid division by zero or very small derivatives
        if dfx.abs() < 1e-10 {
            println!("Zero derivative detected at x = {}", x);
            return Err(RootFindingError::ZeroDerivative);
        }

        // check if the initial guess is already close to a root
        if f(x).0.abs() < tol {
            return Ok(x);
        }

        for _ in 0..max_iter {
            let (fx, dfx) = f(x);

            // avoid division by zero or very small derivatives
            if dfx.abs() < 1e-10 {
                println!("Zero derivative detected at x = {}", x);
                return Err(RootFindingError::ZeroDerivative);
            }

            let new_x = x - fx / dfx;
            if (new_x - x).abs() < tol {
                return Ok(new_x);
            }
            x = new_x
        }
        Err(RootFindingError::NoConvergence)
    }
}

pub struct SecantMethod {
    pub x0: f64,
    pub x1: f64,
}

impl RootFinding for SecantMethod {
    type Output = f64;

    fn find_root<F>(&self, f: F, tol: f64, max_iter: usize) -> Result<f64, RootFindingError>
    where
        F: Fn(f64) -> f64,
    {
        let (mut x0, mut x1) = (self.x0, self.x1);

        for _ in 0..max_iter {
            let fx0 = f(x0);
            let fx1 = f(x1);

            // avoid division by zero or very small derivatives
            if (fx1 - fx0).abs() < 1e-10 {
                return Err(RootFindingError::ZeroDerivative);
            }

            let x2 = x1 - fx1 * (x1 - x0) / (fx1 - fx0);

            if (x2 - x1).abs() < tol {
                return Ok(x2);
            }

            x0 = x1;
            x1 = x2;
        }
        Err(RootFindingError::NoConvergence)
    }
}

pub struct BrentsMethod {
    pub a: f64,
    pub b: f64,
}

// impl RootFinding for BrentsMethod {
//     type Output = f64;

//     fn find_root<F>(&self, f: F, tol: f64, max_iter: usize) -> Result<f64, RootFindingError>
//     where
//         F: Fn(f64) -> f64,
//     {
//         let (mut a, mut b) = (self.a, self.b);
//         let mut c = a;
//         let mut fa = f(a);
//         let mut fb = f(b);
//         let mut fc = fa;

//         // no root interval
//         if fa * fb > 0.0 {
//             return Err(RootFindingError::InvalidInterval);
//         }

//         for _ in 0..max_iter {
//             // no root interval
//             if fa * fb > 0.0 {
//                 return Err(RootFindingError::InvalidInterval);
//             }
//             if fb.abs() < fc.abs() {
//                 std::mem::swap(&mut a, &mut b);
//                 std::mem::swap(&mut fa, &mut fb);
//                 std::mem::swap(&mut c, &mut a);
//                 fc = fa;
//             }

//             let tol_act = 2.0 * tol * b.abs() + tol;
//             let m = 0.5 * (c - b);

//             if m.abs() <= tol_act || fb.abs() < tol {
//                 return Ok(b);
//             }

//             let mut p;
//             let mut q;

//             if (a - c).abs() > tol_act {
//                 let s = fb / fa;
//                 if a == c {
//                     p = 2.0 * m * s;
//                     q = 1.0 - s;
//                 } else {
//                     let r = fa / fb;
//                     let t = fb / fc;
//                     p = s * (2.0 * m * r * (r - t) - (b - a) * (t - 1.0));
//                     q = (r - 1.0) * (t - 1.0) * (s - 1.0);
//                 }

//                 if p > 0.0 {
//                     q = -q
//                 }
//                 p = p.abs();

//                 if 2.0 * p < (3.0 * m * q - (tol_act * q).abs()) {
//                     a = b;
//                     fa = fb;
//                     b += p / q;
//                 } else {
//                     c = a;
//                     fc = fa;
//                     a = b;
//                     fa = fb;
//                     b += m;
//                 }
//             } else {
//                 c = a;
//                 fc = fa;
//                 a = b;
//                 fa = fb;
//                 b += m;
//             }

//             fb = f(b);
//         }
//         Err(RootFindingError::NoConvergence)
//     }
// }

impl RootFinding for BrentsMethod {
    type Output = f64;
    fn find_root<F>(&self, f: F, tol: f64, max_iter: usize) -> Result<f64, RootFindingError>
    where
        F: Fn(f64) -> Self::Output,
    {
        let (a, b) = (self.a, self.b);
        let fa = f(a);
        let fb = f(b);

        // check that the function changes sign
        if fa * fb > 0.0 {
            return Err(RootFindingError::InvalidInterval);
        }

        let root = find_root_brent(
            a,
            b,
            |x| f(x),
            &mut SimpleConvergency { eps: tol, max_iter },
        );
        match root {
            Ok(r) => Ok(r),
            Err(_) => Err(RootFindingError::NoConvergence),
        }
    }
}
