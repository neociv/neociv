use regex::*;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use neociv_macros::StateTable;

use crate::cell::improvement::Improvement;

pub type CityKey = String;

lazy_static! {
    pub static ref VALID_CITY_KEY: Regex =
        Regex::new(r"^[a-zA-Z0-9]+\.[a-zA-Z0-9]+(?:\.[a-zA-Z0-9])*\[\d+\]<\d+>$").unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize, SerdeDiff, StateTable)]
pub struct NeocivCity {
    pub key: CityKey,
    pub title: String,
    pub capital: bool,
    pub improvement: Improvement,
}
