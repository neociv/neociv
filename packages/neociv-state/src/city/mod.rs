use regex::*;

use crate::cell::improvement::{ Improvement, ImprovementType };

pub type CityKey = String;

lazy_static! {
    pub static ref VALID_CITY_KEY: Regex = Regex::new(r"^[a-zA-Z0-9]+\.[a-zA-Z0-9]+(?:\.[a-zA-Z0-9])*\[\d+\]<\d+>$").unwrap();
}

pub struct NeocivCity {
    pub key: CityKey,
    pub title: String,
    pub capital: bool,
}

impl Improvement for NeocivCity {
    fn improvement_type() -> ImprovementType {
       ImprovementType::Building 
    }

    fn improvement_content() -> String {
        String::from("city")
    }

    fn improvement_integrity() -> f32 {
        100.0
    }
}
