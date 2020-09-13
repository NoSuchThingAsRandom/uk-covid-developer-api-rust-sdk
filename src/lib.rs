extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::collections::HashMap;
use std::fmt;

use regex::Regex;
use reqwest::{Client, ClientBuilder};
use serde_json::Value;

use structures::Filters;

use crate::structures::{ApiError, ApiErrorKind, AreaType, Structures};

mod structures;

/// Coronavirus (COVID-19) Dashboard - API Service
/// ==============================================
/// Software Development Kit (SDK)
/// ------------------------------
/// This is a Rust SDK for the COVID-19 API
/// Created by Sam Ralph
/// The ENDPOINT for the data provided using this SDK is:
///     https://api.coronavirus.data.gov.uk/v1/data
/// License: MIT

pub const ENDPOINT: &str = "https://api.coronavirus.data.gov.uk/v1/data";

#[derive(Default)]
pub struct Cov19api {
    filters: HashMap<String, String>,
    structure: HashMap<String, String>,
    client: Client,
}

impl Cov19api {
    /// Instantiates a new Cov19api instance
    /// And builds a reqwest client
    /// Panics if the TLS backend cannot be initialized, or the resolver cannot load the system configuration.
    pub fn new() -> Cov19api {
        let mut client_builder = ClientBuilder::new().gzip(true);
        let mut client = client_builder.build().unwrap();
        Cov19api {
            filters: Default::default(),
            structure: Default::default(),
            client,
        }
    }
    ///Resets all filters and requested structures
    pub fn clear(&mut self) {
        self.filters.clear();
        self.structure.clear();
    }
    /// Adds the given filter to the request
    /// Provided it is a valid field, and the value is correct
    /// Only checks areaType and date format
    ///
    /// Panics if the regex data checker is broken
    pub fn set_filter_string(
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

    /// Adds the given filter to the request
    /// Provided it is a valid field, and the value is correct
    /// Only checks areaType and date format
    /// Panics if the regex data checker is broken
    pub fn set_filter_enum(
        &mut self,
        filter_name: Filters,
        filter_value: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //TODO Create contains fn
        if filter_name == Filters::areaType && !AreaType::to_vec().contains(&filter_value) {
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
        if filter_name == Filters::date && date_regex.is_match(&filter_value.as_str()) {
            let msg = String::from("Invalid date provided!\nNeeds to be in the format YYYY-MM-DD");
            println!("{}", msg);
            return Err(Box::new(ApiError {
                kind: ApiErrorKind::InvalidFilterValue,
                msg,
            }));
        }
        self.filters
            .insert(format!("{:?}", filter_name), filter_value);
        Ok(())
    }

    /// Requests a metric from the covid api
    /// Must be one of Structures
    ///
    /// Will set the metric name that is returned, to the given metric name
    pub fn set_structure_string(&mut self, structure_name: String) {
        self.set_structure_string_rename(structure_name.clone(), structure_name);
    }

    /// Requests a metric from the covid api
    /// Must be one of Structures
    ///
    /// Will set the metric name that is returned, to the given return_name value
    pub fn set_structure_string_rename(
        &mut self,
        structure_name: String,
        return_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !Structures::to_vec().contains(&structure_name) {
            let msg = format!(
                "Invalid structure name provided!\nNeeds to be one of: {}",
                Structures::to_string()
            );
            println!("{}", msg);
            return Err(Box::new(ApiError {
                kind: ApiErrorKind::InvalidStructure,
                msg,
            }));
        }
        self.structure.insert(structure_name, return_name);
        Ok(())
    }

    /// Adds a metric to get from the api
    /// Will set the return metric name, to the structure_name provided
    pub fn set_structure_enum(
        &mut self,
        structure_name: Structures,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.structure.insert(
            format!("{:?}", structure_name),
            format!("{:?}", structure_name),
        );
        Ok(())
    }

    pub async fn send_request(&mut self) -> Result<Value, Box<dyn std::error::Error>> {
        let mut url: String = String::from(ENDPOINT);
        if !self.filters.is_empty() {
            url.push_str("?filters=");
            let mut index = 0;
            for (filter_name, filter_value) in self.filters.iter() {
                url.push_str(filter_name.as_str());
                url.push_str("=");
                url.push_str(filter_value.as_str());
                if index + 1 != self.filters.len() {
                    url.push_str(";");
                }
                index += 1;
            }
        }
        if !self.structure.is_empty() {
            url.push_str(if self.filters.is_empty() { "?" } else { "&" });
            url.push_str("structure={");
            let mut index = 0;
            for (structure_name, structure_value) in self.structure.iter() {
                url.push_str("\"");
                url.push_str(structure_name.as_str());
                url.push_str("\":\"");
                url.push_str(structure_value.as_str());
                url.push_str("\"");
                if index + 1 != self.structure.len() {
                    url.push_str(",");
                }
                index += 1;
            }
            url.push_str("}");
        }
        url.push_str("&format=json");
        url.push_str("&page=1");
        println!("Url: {}", url);
        let response = self.client.get(url.as_str()).send().await?;
        let json = response.json::<Value>().await?;
        println!("{:#?}", json);
        println!("{:#?}", json["data"]);
        Ok(json)
    }
}

pub fn test() {
    main();
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = Cov19api::default();
    api.set_filter_enum(Filters::areaType, format!("{:?}", AreaType::nation));
    api.set_filter_enum(Filters::areaName, String::from("england"));
    api.set_structure_enum(Structures::maleCases);
    api.set_structure_enum(Structures::femaleCases);
    println!("Built request");
    api.send_request().await.unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
