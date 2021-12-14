use serde::{Deserialize, Serialize};
use chrono::{Date, Local};

#[derive(Debug, Deserialize, Serialize)]
struct Contact {
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Membership {
    id: i32,
    location: Vec<String>,
    start: Date<Local>,
    exp: Option<Date<Local>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub contact: Contact,
    // pub email: String,
    pub password: String, // TODO make hash for this
    pub fname: String,
    pub lname: String,
    pub username: Option<String>,
    // pub phone: Option<String>,
    pub membership: Option<Membership>,
    
}