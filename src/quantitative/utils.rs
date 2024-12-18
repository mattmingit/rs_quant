use crate::quantitative::returns::Return;
use crate::quantitative::traits::{AssetReturn, ReturnDatetime};
use chrono::NaiveDate;
use std::collections::HashMap;

pub fn align_returns<T>(returns1: &[T], returns2: &[T]) -> Result<(Vec<f64>, Vec<f64>), String>
where
    T: ReturnDatetime + AssetReturn,
{
    let mut ret_map: HashMap<String, f64> = HashMap::new();

    // Parse returns2 dates and return values
    for ret in returns2 {
        let date_str = ret.datetime().split_whitespace().next().unwrap_or("");
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => {
                ret_map.insert(date.to_string(), ret.asset_return());
            }
            Err(e) => {
                eprintln!(
                    "Error parsing return dataset 2 date '{}': {:?}",
                    date_str, e
                );
                return Err("Error: Invalid returns dataset 2 date format.".to_string());
            }
        }
    }

    let mut aligned_ret1: Vec<f64> = Vec::new();
    let mut aligned_ret2: Vec<f64> = Vec::new();

    // Parse the returns1 dates and check for alignment with returns2 dates
    for ret in returns1 {
        let date_str = ret.datetime().split_whitespace().next().unwrap_or("");
        match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(date) => {
                if let Some(&ret2) = ret_map.get(&date.to_string()) {
                    aligned_ret1.push(ret.asset_return());
                    aligned_ret2.push(ret2);
                }
            }
            Err(e) => {
                eprintln!(
                    "Error parsing return dataset 1 date '{}': {:?}",
                    date_str, e
                );
                return Err("Error: Invalid returns dataset 1 date format.".to_string());
            }
        }
    }

    if aligned_ret1.is_empty() || aligned_ret2.is_empty() {
        return Err(
            "Error: No overlapping dates between return dataset 1 and return dataset 2".to_string(),
        );
    }

    Ok((aligned_ret1, aligned_ret2))
}
