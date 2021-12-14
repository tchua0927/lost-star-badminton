
use serde::{Deserialize, Serialize};
use chrono::{Weekday, NaiveTime};

use crate::data::models::court::Court;

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    addr_num: i32,
    street: String,
    apt_num: Option<i32>,
    city: String,
    state_province: String,
    zip: i32,
    nation: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenClosePair {
    open: NaiveTime,
    close: NaiveTime
}

#[derive(Debug, Deserialize, Serialize)]
struct Week_Day {
    day: Weekday,
    times: Vec<OpenClosePair>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub address: Address,
    pub courts: Vec<Court>,
    pub schedule: Vec<Week_Day>
    
}