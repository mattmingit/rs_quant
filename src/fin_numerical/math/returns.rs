use std::error::Error;
use yahoo_finance_api::Quote;

pub enum ReturnType {
    Logarithmic,
    Arithmetic,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Return {
    timestamp: i64,
    ret: f64,
}

pub fn compute_returns(
    data: Vec<Quote>,
    return_type: ReturnType,
) -> Result<Vec<Return>, Box<dyn Error>> {
    if data.len() < 2 {
        return Err("Not enough data to calculate returns.".into());
    }

    let mut ret = Vec::new();
    for pair in data.windows(2) {
        let prev = &pair[0];
        let curr = &pair[1];

        if prev.adjclose.abs() < f64::EPSILON || curr.adjclose.abs() < f64::EPSILON {
            return Err("Error: adjclose value is zero.".into());
        }

        let ret_value = match return_type {
            ReturnType::Arithmetic => (curr.adjclose / prev.adjclose) - 1.0,
            ReturnType::Logarithmic => (curr.adjclose / prev.adjclose).ln(),
        };

        ret.push(Return {
            timestamp: curr.timestamp as i64,
            ret: ret_value,
        })
    }

    Ok(ret)
}
