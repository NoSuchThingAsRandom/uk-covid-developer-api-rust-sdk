use std::fmt;
use std::fmt::Write;

use crate::strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct ApiError {
    pub kind: ApiErrorKind,
    pub msg: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiErrorKind {
    InvalidFilter,
    InvalidFilterValue,
    InvalidStructure,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type:{:?}, Msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for ApiError {}

#[derive(EnumIter, Debug, PartialEq)]
/// All valid Filter fields
///
/// Given by: https://coronavirus.data.gov.uk/developers-guide#params-filters
///
/// Last update: 2020/09/12
///
/// Field Descriptions:
///
/// <br> areaType - Area type as string
/// <br> areaName - Area name as string
/// <br> areaCode - Area Code as string
/// <br> date - Date as string [YYYY-MM-DD]
pub enum Filters {
    areaType,
    areaName,
    areaCode,
    date,
}

impl Filters {
    /// Exports all valid filter fields to a Vec of strings
    pub fn to_vec() -> Vec<String> {
        let mut filter_str = Vec::new();
        for filter in Filters::iter() {
            filter_str.push(format!("{:?}", filter));
        }
        filter_str
    }
    /// Creates a string containing all valid filter fields, on separate lines
    pub fn to_string() -> String {
        let mut filter_str = String::new();
        for filter in Filters::iter() {
            filter_str.push_str(&format!("{:?}", filter));
        }
        filter_str
    }
    /// Creates a string containing all filter field descriptions, on separate lines
    pub fn field_descriptions() -> String {
        String::from(
            "\n areaType - Area type as string
\n areaName - Area name as string
\n areaCode - Area Code as string
\n date - Date as string [YYYY-MM-DD]",
        )
    }
}

#[derive(EnumIter, Debug)]
/// All valid AreaType values
///
/// Given by: https://coronavirus.data.gov.uk/developers-guide#params-structure
///
/// Last update: 2020/09/12
///
/// Field Descriptions:
///
/// <br> overview - Overview data for the United Kingdom
/// <br> nation - Nation data (England, Northern Ireland, Scotland, and Wales)
/// <br> region - Region data
/// <br> nhsRegion - NHS Region data
/// <br> utla - Upper-tier local authority data
/// <br> ltla - Lower-tier local authority data
pub enum AreaType {
    overview,
    nation,
    region,
    nhsRegion,
    utla,
    ltla,
}

impl AreaType {
    /// Exports all valid AreaType values to a Vec of strings
    pub fn to_vec() -> Vec<String> {
        let mut area_type_str = Vec::new();
        for areaType in AreaType::iter() {
            area_type_str.push(format!("{:?}", areaType));
        }
        area_type_str
    }
    /// Creates a string containing all valid AreaType values, on separate lines
    pub fn to_string() -> String {
        let mut area_type_str = String::new();
        for areaType in AreaType::iter() {
            area_type_str.push_str(&format!("{:?}", areaType));
        }
        area_type_str
    }
    /// Creates a string containing all structure field descriptions, on separate lines
    pub fn field_description() -> String {
        String::from(
            "\n overview - Overview data for the United Kingdom
\n nation - Nation data (England, Northern Ireland, Scotland, and Wales)
\n region - Region data
\n nhsRegion - NHS Region data
\n utla - Upper-tier local authority data
\n ltla - Lower-tier local authority data",
        )
    }
}

#[derive(EnumIter, Debug)]
/// All valid structure fields
///
/// Given by: https://coronavirus.data.gov.uk/developers-guide#params-structure
///
/// Last update: 2020/09/12
///
/// Field Descriptions:
///
/// <br> areaType - Area type as string
/// <br> areaName - Area name as string
/// <br> areaCode - Area Code as string
/// <br> date - Date as string [YYYY-MM-DD]
/// <br> hash - Unique ID as string
/// <br>
/// <br> newCasesByPublishDate - New cases by publish date
/// <br> cumCasesByPublishDate - Cumulative cases by publish date
/// <br> cumCasesBySpecimenDateRate - Rate of cumulative cases by publish date per 100k resident population
/// <br> newCasesBySpecimenDate - New cases by specimen date
/// <br> cumCasesBySpecimenDateRate - Rate of cumulative cases by specimen date per 100k resident population
/// <br> cumCasesBySpecimenDate - Cumulative cases by specimen date
/// <br> maleCases - Male cases (by age)
/// <br> femaleCases - Female cases (by age)
/// <br>
/// <br> newPillarOneTestsByPublishDate - New pillar one tests by publish date
/// <br> cumPillarOneTestsByPublishDate - Cumulative pillar one tests by publish date
/// <br> newPillarTwoTestsByPublishDate - New pillar two tests by publish date
/// <br> cumPillarTwoTestsByPublishDate - Cumulative pillar two tests by publish date
/// <br> newPillarThreeTestsByPublishDate - New pillar three tests by publish date
/// <br> cumPillarThreeTestsByPublishDate - Cumulative pillar three tests by publish date
/// <br> newPillarFourTestsByPublishDate - New pillar four tests by publish date
/// <br> cumPillarFourTestsByPublishDate - Cumulative pillar four tests by publish date
/// <br>
/// <br> newAdmissions - New admissions
/// <br> cumAdmissions - Cumulative number of admissions
/// <br> cumAdmissionsByAge - Cumulative admissions by age
/// <br> cumTestsByPublishDate - Cumulative tests by publish date
/// <br> newTestsByPublishDate - New tests by publish date
/// <br> covidOccupiedMVBeds - COVID-19 occupied beds with mechanical ventilators
/// <br> hospitalCases - Hospital cases
/// <br> plannedCapacityByPublishDate - Planned capacity by publish date
/// <br>
/// <br> newDeaths28DaysByPublishDate - Deaths within 28 days of positive test
/// <br> cumDeaths28DaysByPublishDate - Cumulative deaths within 28 days of positive test
/// <br> cumDeaths28DaysByPublishDateRate - Rate of cumulative deaths within 28 days of positive test per 100k resident population
/// <br> newDeaths28DaysByDeathDate - Deaths within 28 days of positive test by death date
/// <br> cumDeaths28DaysByDeathDate - Cumulative deaths within 28 days of positive test by death date
/// <br> cumDeaths28DaysByDeathDateRate - Rate of cumulative deaths within 28 days of positive test by death date per 100k resident population
pub enum Structures {
    areaType,
    areaName,
    areaCode,
    date,
    hash,
    newCasesByPublishDate,
    cumCasesByPublishDate,
    cumCasesBySpecimenDateRate,
    newCasesBySpecimenDate,
    maleCases,
    femaleCases,

    newPillarOneTestsByPublishDate,
    cumPillarOneTestsByPublishDate,

    newPillarTwoTestsByPublishDate,
    cumPillarTwoTestsByPublishDate,

    newPillarThreeTestsByPublishDate,
    cumPillarThreeTestsByPublishDate,

    newPillarFourTestsByPublishDate,
    cumPillarFourTestsByPublishDate,

    newAdmissions,
    cumAdmissions,
    cumAdmissionsByAge,

    newTestsByPublishDate,
    cumTestsByPublishDate,

    covidOccupiedMVBeds,
    hospitalCases,
    plannedCapacityByPublishDate,
    newDeaths28DaysByPublishDate,
    cumDeaths28DaysByPublishDate,
    cumDeaths28DaysByPublishDateRate,
    newDeaths28DaysByDeathDate,
    cumDeaths28DaysByDeathDate,
    cumDeaths28DaysByDeathDateRate,
}

impl Structures {
    /// Exports all valid structure fields to a Vec of strings
    pub fn to_vec() -> Vec<String> {
        let mut structure_str = Vec::new();
        for structure in Structures::iter() {
            structure_str.push(format!("{:?}", structure));
        }
        structure_str
    }
    /// Creates a string containing all valid structure fields, on separate lines
    pub fn to_string() -> String {
        let mut structure_str = String::new();
        for structure in Structures::iter() {
            structure_str.push_str(&format!("{:?}\n", structure));
        }
        structure_str
    }
    /// Creates a string containing all structure field descriptions, on separate lines
    pub fn field_description() -> String {
        String::from("\n areaType - Area type as string
\n areaName - Area name as string
\n areaCode - Area Code as string
\n date - Date as string [ - YYYY-MM-DD - ]
\n hash - Unique ID as string
\n
\n newCasesByPublishDate - New cases by publish date
\n cumCasesByPublishDate - Cumulative cases by publish date
\n cumCasesBySpecimenDateRate - Rate of cumulative cases by publish date per 100k resident population
\n newCasesBySpecimenDate - New cases by specimen date
\n cumCasesBySpecimenDateRate - Rate of cumulative cases by specimen date per 100k resident population
\n cumCasesBySpecimenDate - Cumulative cases by specimen date
\n maleCases - Male cases (by age)
\n femaleCases - Female cases (by age)
\n
\n newPillarOneTestsByPublishDate - New pillar one tests by publish date
\n cumPillarOneTestsByPublishDate - Cumulative pillar one tests by publish date
\n newPillarTwoTestsByPublishDate - New pillar two tests by publish date
\n cumPillarTwoTestsByPublishDate - Cumulative pillar two tests by publish date
\n newPillarThreeTestsByPublishDate - New pillar three tests by publish date
\n cumPillarThreeTestsByPublishDate - Cumulative pillar three tests by publish date
\n newPillarFourTestsByPublishDate - New pillar four tests by publish date
\n cumPillarFourTestsByPublishDate - Cumulative pillar four tests by publish date
\n
\n newAdmissions - New admissions
\n cumAdmissions - Cumulative number of admissions
\n cumAdmissionsByAge - Cumulative admissions by age
\n cumTestsByPublishDate - Cumulative tests by publish date
\n newTestsByPublishDate - New tests by publish date
\n covidOccupiedMVBeds - COVID-19 occupied beds with mechanical ventilators
\n hospitalCases - Hospital cases
\n plannedCapacityByPublishDate - Planned capacity by publish date
\n
\n newDeaths28DaysByPublishDate - Deaths within 28 days of positive test
\n cumDeaths28DaysByPublishDate - Cumulative deaths within 28 days of positive test
\n cumDeaths28DaysByPublishDateRate - Rate of cumulative deaths within 28 days of positive test per 100k resident population
\n newDeaths28DaysByDeathDate - Deaths within 28 days of positive test by death date
\n cumDeaths28DaysByDeathDate - Cumulative deaths within 28 days of positive test by death date
\n cumDeaths28DaysByDeathDateRate - Rate of cumulative deaths within 28 days of positive test by death date per 100k resident population")
    }
}
