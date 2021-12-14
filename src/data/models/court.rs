use serde::{Deserialize, Serialize};
use chrono::{Date,NaiveTime, Local};

#[derive(Debug, Deserialize, Serialize)]
enum CourtType {
    Open,
    Challenge,
    Training,
}


#[derive(Debug, Deserialize, Serialize)]
struct Reservation {
    id: i32,
    email: String,
    court_id: i32,
    date: Date<Local>,
    start: NaiveTime,
    end: NaiveTime,
    paid: bool,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Court {
    pub id: i32,
    pub court_type: CourtType,
    pub reservation: Vec<Reservation>,


}