use regex::*;

use crate::cell::improvement::Improvement;
use crate::state_table;

pub type CityKey = String;

lazy_static! {
    pub static ref VALID_CITY_KEY: Regex =
        Regex::new(r"^[a-zA-Z0-9]+\.[a-zA-Z0-9]+(?:\.[a-zA-Z0-9])*\[\d+\]<\d+>$").unwrap();
}

state_table! {
    pub struct NeocivCity {
        pub key: CityKey,
        pub title: String,
        pub capital: bool,
        pub improvement: Improvement,
    }
}
