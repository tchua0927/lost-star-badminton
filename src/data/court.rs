use serde::{Deserialize, Serialize};

enum CourtType {
    Open,
    Challenge,
    Training,
}

pub struct Court {
    pub id: i32,
    pub court_type: CourtType,

}