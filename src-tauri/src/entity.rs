use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Param {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Form {
    pub field: String,
    pub value: String,
    pub value_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub protocol: String,
    pub method: String,
    pub host: String,
    pub port: String,
    pub query: Vec<Param>,
    pub params: Vec<Param>,
    pub path: String,
    pub headers: Vec<Header>,
    pub body_type: String,
    pub form: Vec<Form>,
    pub json: String,
}
