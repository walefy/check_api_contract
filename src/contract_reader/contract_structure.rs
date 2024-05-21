use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum BodyType {
    Number(i32),
    String(String),
    HashMap(HashMap<String, BodyType>),
    Array(Vec<BodyType>),
    Null,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Expect {
    pub status: u16,
    pub body: BodyType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub method_type: String,
    pub endpoint: String,
    pub body: BodyType,
    pub headers: Option<HashMap<String, String>>,
    pub expect: Expect,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub base_url: String,
    pub timeout: u16,
    pub methods: Vec<Method>,
}
