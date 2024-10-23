use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BMC {
    pub vendor: String,
    pub address: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub username: String,
    pub password: String,

    pub bmc: Vec<BMC>,
}
