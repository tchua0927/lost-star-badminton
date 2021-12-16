
use serde::{Deserialize, Serialize};
use mongodb::bson::DateTime;

use crate::data::models::court::Court;

#[derive(Debug, Deserialize, Serialize)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday, 
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    addr_num: i32,
    street: String,
    apt_num: Option<i32>,
    city: String,
    state_province: String,
    zip: i32,
    nation: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenClosePair {
    open: DateTime,
    close: DateTime
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DayTime {
    day: Weekday,
    times: Vec<OpenClosePair>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub address: Address,
    pub courts: Vec<Court>,
    pub schedule: Vec<DayTime>
    
}