use crate::data::quotes::QuoteItem;
use std::error::Error;

pub enum ReturnType {
    Logarithmic,
    Arithmetic,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Return {
    pub datetime: String,
    pub asset_return: f64,
}

pub fn compute_returns(
    data: Vec<QuoteItem>,
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
            datetime: curr.datetime.clone(),
            asset_return: ret_value,
        })
    }

    Ok(ret)
}
