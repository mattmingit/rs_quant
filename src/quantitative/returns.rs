use std::collections::HashMap;

use ndarray::Array1;
use thiserror::Error;

use crate::commons::parser::round_to_four;
use crate::data::yahoo::{MultiQuoteItem, QuoteItem};

#[derive(Debug, Error)]
pub enum ReturnsError {
    #[error("Empty Input.")]
    EmptyInput,
    #[error("Inconsistent data: vector length must be more than one, only {0}")]
    LengthError(usize),
    #[error("Method not implemented for {0}")]
    NotImplementedFor(String),
}

pub enum ReturnType {
    Absolute,
    Arithmetic,
    Logarithmic,
}

pub trait Returns {
    fn returns(&self, r_type: ReturnType) -> Result<Array1<(String, f64)>, ReturnsError>;
    fn cumulative_returns(&self, r_type: ReturnType)
        -> Result<Array1<(String, f64)>, ReturnsError>;
    fn returns_multiquote(
        &self,
        r_type: ReturnType,
    ) -> Result<Array1<(String, HashMap<String, f64>)>, ReturnsError>;
}

impl Returns for Array1<QuoteItem> {
    // compute returns of data contained into an array composed by QuoteItem. Returns an array of tuples (datetime, return value)
    fn returns(&self, r_type: ReturnType) -> Result<Array1<(String, f64)>, ReturnsError> {
        if self.len() < 2 {
            return Err(ReturnsError::LengthError(self.len()));
        }

        let mut r = Array1::from_elem(self.len() - 1, (String::new(), 0.0));
        for i in 1..self.len() {
            if self[i - 1].adjclose == 0.0 {
                return Err(ReturnsError::LengthError(self.len()));
            }

            let r_val = match r_type {
                ReturnType::Arithmetic => (&self[i].adjclose / &self[i - 1].adjclose) - 1.0,
                ReturnType::Logarithmic => (&self[i].adjclose / &self[i - 1].adjclose).ln(),
                ReturnType::Absolute => &self[i].adjclose / &self[i - 1].adjclose,
            };
            r[i - 1] = (self[i].datetime.clone(), round_to_four(r_val));
        }
        Ok(r)
    }

    fn cumulative_returns(
        &self,
        r_type: ReturnType,
    ) -> Result<Array1<(String, f64)>, ReturnsError> {
        let ret = self.returns(r_type)?;
        let mut c_ret = Array1::from_elem(self.len() - 1, (String::new(), 0.0));

        let mut c_val = 0.0;
        for i in 0..ret.len() {
            c_val += ret[i].1;
            c_ret[i] = (ret[i].0.clone(), round_to_four(c_val))
        }
        Ok(c_ret)
    }

    #[allow(unused_variables)]
    fn returns_multiquote(
        &self,
        r_type: ReturnType,
    ) -> Result<Array1<(String, HashMap<String, f64>)>, ReturnsError> {
        Err(ReturnsError::NotImplementedFor(
            "Array1<QuoteItem>".to_string(),
        ))
    }
}

impl Returns for Array1<MultiQuoteItem> {
    #[allow(unused_variables)]
    fn returns(&self, r_type: ReturnType) -> Result<Array1<(String, f64)>, ReturnsError> {
        Err(ReturnsError::NotImplementedFor(
            "Array1<MultiQuoteItem>".to_string(),
        ))
    }

    #[allow(unused_variables)]
    fn cumulative_returns(
        &self,
        r_type: ReturnType,
    ) -> Result<Array1<(String, f64)>, ReturnsError> {
        Err(ReturnsError::NotImplementedFor(
            "Array1<MultiQuoteItem>".to_string(),
        ))
    }

    fn returns_multiquote(
        &self,
        r_type: ReturnType,
    ) -> Result<Array1<(String, HashMap<String, f64>)>, ReturnsError> {
        if self.len() < 2 {
            return Err(ReturnsError::LengthError(self.len()));
        }

        let mut r = Array1::from_elem(self.len() - 1, (String::new(), HashMap::new()));
        for i in 1..self.len() {
            let prev = &self[i - 1];
            let curr = &self[i];

            let mut ret_map = HashMap::new();
            for (t, curr_p) in &curr.prices {
                if let Some(prev_p) = prev.prices.get(t) {
                    let r_val = match r_type {
                        ReturnType::Arithmetic => (curr_p / prev_p) - 1.0,
                        ReturnType::Logarithmic => (curr_p / prev_p).ln(),
                        ReturnType::Absolute => curr_p / prev_p,
                    };
                    ret_map.insert(t.clone(), round_to_four(r_val));
                }
            }
            r[i - 1] = (curr.date.clone(), ret_map);
        }
        Ok(r)
    }
}
