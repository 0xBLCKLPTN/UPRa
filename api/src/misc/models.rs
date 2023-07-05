use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Phones {
    pub id: Option<i64>,
    pub phone_number: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comments {
    pub id: Option<i64>,
    pub phone_id: i64,
    pub comment: String,
}
