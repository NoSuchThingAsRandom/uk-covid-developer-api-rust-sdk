extern crate strum;
#[macro_use]
extern crate strum_macros;

use regex::Regex;
use std::collections::HashMap;
use std::fmt;
mod structures;
use crate::structures::AreaType;
use structures::Filters;

/// Coronavirus (COVID-19) Dashboard - API Service
/// ==============================================
/// Software Development Kit (SDK)
/// ------------------------------
/// This is a Rust SDK for the COVID-19 API, created by
/// Public Health England
///
/// The ENDPOINT for the data provided using this SDK is:
///     https://api.coronavirus.data.gov.uk/v1/data
///     Coronavirus (COVID-19) in the UK`: http://coronavirus.data.gov.uk/
/// License: MIT

pub const ENDPOINT: &str = "https://api.coronavirus.data.gov.uk/v1/data";

#[derive(Debug, Clone)]
pub struct ApiError {
    kind: ApiErrorKind,
    msg: String,
}

#[derive(Debug, Clone)]
pub enum ApiErrorKind {
    InvalidFilter,
    InvalidFilterValue,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type:{:?}, Msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for ApiError {}

#[derive(Default)]
pub struct Cov19api {
    filters: HashMap<String, String>,
    structure: HashMap<String, String>,
}

impl Cov19api {
    pub fn set_filter(
        &mut self,
        filter_name: String,
        filter_value: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !Filters::to_vec().contains(&filter_name) {
            let msg = format!(
                "Invalid filter name provided!\nNeeds to be one of: {}",
                Filters::to_string()
            );
            println!("{}", msg);
            return Err(Box::new(ApiError {
                kind: ApiErrorKind::InvalidFilter,
                msg,
            }));
        }
        if filter_name.eq("areaType") && !AreaType::to_vec().contains(&filter_value) {
            let msg = format!(
                "Invalid area type provided!\nNeeds to be one of: {:?}",
                AreaType::to_string()
            );
            println!("{}", msg);
            return Err(Box::new(ApiError {
                kind: ApiErrorKind::InvalidFilterValue,
                msg,
            }));
        }
        let date_regex =
            Regex::new(r"^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01])$").unwrap();
        if filter_name.eq("date") && date_regex.is_match(&filter_value.as_str()) {
            let msg = String::from("Invalid date provided!\nNeeds to be in the format YYYY-MM-DD");
            println!("{}", msg);
            return Err(Box::new(ApiError {
                kind: ApiErrorKind::InvalidFilterValue,
                msg,
            }));
        }
        self.filters.insert(filter_name, filter_value);
        Ok(())
    }
}

pub fn test() {
    //Filters::test();
    //println!("{:?}", enums::Filters::areaCode);
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
