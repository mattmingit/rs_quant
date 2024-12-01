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

#[allow(dead_code)]
#[derive(Debug)]
pub struct CumulativeReturn {
    pub datetime: String,
    pub asset_cumulative_returns: f64,
}

// Compute the calculation of asset returns: returns calculated can be arithmetic or logarithmic
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

// Compute the calculation of asset cumulative returns: the argument of the function is a Vec of returns (computed by precedent function)
pub fn compute_cumulative_returns(
    returns: Vec<Return>,
) -> Result<Vec<CumulativeReturn>, Box<dyn Error>> {
    let mut cumulative = Vec::with_capacity(returns.len());
    let mut cum_ret = 1.0;

    for r in returns {
        cum_ret *= 1.0 + r.asset_return;
        cumulative.push(CumulativeReturn {
            datetime: r.datetime.clone(),
            asset_cumulative_returns: cum_ret - 1.0,
        });
    }
    Ok(cumulative)
}
