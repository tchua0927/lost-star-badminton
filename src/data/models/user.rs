use serde::{Deserialize, Serialize};
use mongodb::{bson::DateTime};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Membership {
    id: i32,
    location: Vec<String>,
    start: DateTime,
    exp: Option<DateTime>,
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