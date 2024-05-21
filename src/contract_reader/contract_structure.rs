use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum BodyType {
    Number(i32),
    String(String),
    HashMap(HashMap<String, BodyType>),
    Array(Vec<BodyType>),
    Null,
}

impl BodyType {
    pub fn to_string(&self) -> Option<String> {
        match self {
            BodyType::Number(v) => Some(v.to_string()),
            BodyType::String(v) => Some(v.to_string()),
            BodyType::HashMap(v) => Some(serde_json::to_string(v).unwrap()),
            BodyType::Array(v) => Some(serde_json::to_string(v).unwrap()),
            BodyType::Null => None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Expect {
    pub status: u16,
    pub body: BodyType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub method_type: HttpMethod,
    pub endpoint: String,
    pub body: Option<BodyType>,
    pub headers: Option<HashMap<String, String>>,
    pub expect: Expect,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub base_url: String,
    pub timeout: u64,
    pub methods: Vec<Method>,
}
