
use serde::{Deserialize, Serialize};
use mongodb::{bson::DateTime};
use passwords::{hasher};

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

impl User {
    pub fn new(
        contact: Contact,
        password: String,
        fname: String,
        lname: String,
        username: Option<String>,
        membership: Option<Membership>

    ) -> Self {
        let salt = hasher::gen_salt();
        let hashed_pwd = hasher::bcrypt(10, &salt, &password).unwrap();

        User {
            contact: contact,
            // password: hashed_pwd,
            password: password,
            fname: fname,
            lname: lname,
            username: username,
            membership: membership,
        }
    }
    
}