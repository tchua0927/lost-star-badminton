use serde::{Deserialize, Serialize};

pub struct Address {
    pub addr_num: i32,
    pub street: String,
    pub apt_num: Option<i32>,
    pub city: String,
    pub state_province: String,
    pub zip: i32,
    pub nation: String,
}

pub struct Location {
    pub address: Address,
    
}