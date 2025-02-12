use ndarray::Array1;
use num::{FromPrimitive, Num, ToPrimitive};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SequenceErr {
    #[error("Step must not be equal to zero. Use rep() instead.")]
    InvalidStep,
    #[error("Conversion to usize failed")]
    UsizeConversionFailed,
    #[error("Invalid parameters")]
    InvalidParams,
    #[error("Floating-point conversion failed")]
    FloatConversionFailed,
    #[error("Empty input.")]
    EmptyInput,
}

// trait to generate sequences of numbers
pub trait Sequence<T: PartialOrd + Copy + Num + FromPrimitive + ToPrimitive> {
    // generate a sequence of numbers from `s` (start) to `e` (end) with a step size of `step`
    fn seq(s: T, e: T, step: T) -> Result<Array1<T>, SequenceErr>;

    // repeat a number `x`, `n` times
    fn rep(x: T, n: usize) -> Result<Array1<T>, SequenceErr>;

    // generate a sequence of numbers from `s` (start) to `e` (end) with `n` elements (linearly spaced)
    fn lin_space(s: T, e: T, n: usize) -> Result<Array1<T>, SequenceErr>;

    // generate a sequence of numbers from `s` (start) to `e` (end) with `n` elements (logarithmically spaced)
    fn log_space(s: T, e: T, n: usize) -> Result<Array1<T>, SequenceErr>;

    // compute cumulative sum of an array
    fn cumsum(a: &Array1<T>) -> Result<Array1<T>, SequenceErr>;
}

impl<T> Sequence<T> for Array1<T>
where
    T: PartialOrd + Copy + Num + FromPrimitive + ToPrimitive,
{
    fn seq(s: T, e: T, step: T) -> Result<Array1<T>, SequenceErr> {
        if step == T::zero() {
            return Err(SequenceErr::InvalidStep);
        }

        let c = ((e - s) / step)
            .to_usize()
            .ok_or(SequenceErr::UsizeConversionFailed)?;
        let mut v = Vec::with_capacity(c);
        let mut x = s;

        while x <= e {
            v.push(x);
            x = x + step;
        }
        Ok(Array1::from(v))
    }

    fn rep(x: T, n: usize) -> Result<Array1<T>, SequenceErr> {
        Ok(Array1::from_elem(n, x))
    }

    fn lin_space(s: T, e: T, n: usize) -> Result<Array1<T>, SequenceErr> {
        if s == e || n == 0 {
            return Err(SequenceErr::InvalidParams);
        }

        let s_f64 = s.to_f64().ok_or(SequenceErr::FloatConversionFailed)?;
        let e_f64 = e.to_f64().ok_or(SequenceErr::FloatConversionFailed)?;

        let arr = Array1::linspace(s_f64, e_f64, n);
        let r = arr
            .mapv(|v| T::from_f64(v).ok_or(SequenceErr::FloatConversionFailed))
            .into_iter()
            .collect::<Result<Array1<T>, _>>()?;

        Ok(r)
    }

    fn log_space(s: T, e: T, n: usize) -> Result<Array1<T>, SequenceErr> {
        if s == e || n == 0 {
            return Err(SequenceErr::InvalidParams);
        }

        let b_f64 = 10.0;
        let s_f64 = s.to_f64().ok_or(SequenceErr::FloatConversionFailed)?;
        let e_f64 = e.to_f64().ok_or(SequenceErr::FloatConversionFailed)?;

        let arr = Array1::logspace(b_f64, s_f64, e_f64, n);
        let r = arr
            .mapv(|v| T::from_f64(v).ok_or(SequenceErr::FloatConversionFailed))
            .into_iter()
            .collect::<Result<Array1<T>, _>>()?;

        Ok(r)
    }

    fn cumsum(a: &Array1<T>) -> Result<Array1<T>, SequenceErr> {
        if a.is_empty() {
            return Err(SequenceErr::EmptyInput);
        }

        if a.iter().all(|&x| x == T::zero()) {
            return Ok(a.clone());
        }

        let mut r = Array1::zeros(a.len());
        let mut s = T::zero();
        for (i, &v) in a.iter().enumerate() {
            s = s + v;
            r[i] = s;
        }
        Ok(r)
    }
}
