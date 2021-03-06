use chrono::Local;
use djangohashers::{check_password, make_password};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub email: String,
    pub phone: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Membership {
    pub id: i32,
    pub location: Vec<String>,
    pub start: DateTime,
    // pub start: String,
    pub exp: Option<DateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub contact: Contact,
    pub password: String, // TODO make hash for this
    pub fname: String,
    pub lname: String,
    pub username: Option<String>,
    pub membership: Option<Membership>,
}

impl User {
    pub fn new(
        contact: Contact,
        password: String,
        fname: String,
        lname: String,
        username: Option<String>, 
        membership: Option<Membership>,
    ) -> Self {
        let hashed_pwd = make_password(password.as_str());
        User {
            contact: contact,
            // password: hashed_pwd,
            password: hashed_pwd,
            fname: fname,
            lname: lname,
            username: username,
            membership: membership,
        }
    }
}
