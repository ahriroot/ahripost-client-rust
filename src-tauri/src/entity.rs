use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub protocol: String,
    pub method: String,
    pub host: String,
    pub port: String,
}
