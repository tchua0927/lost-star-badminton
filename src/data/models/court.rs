use serde::{Deserialize, Serialize};
// use chrono::{Date,NaiveTime, Local};
use mongodb::{bson::DateTime};

#[derive(Debug, Deserialize, Serialize)]
pub enum CourtType {
    Open,
    Challenge,
    Training,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Reservation {
    id: i32,
    email: String,
    court_id: i32,
    time: DateTime,
    duration: i32,
    paid: bool,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Court {
    pub id: i32,
    pub court_type: CourtType,
    pub reservation: Vec<Reservation>,
}