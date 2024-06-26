use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
    Put,
    Head,
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

impl fmt::Display for BodyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BodyType::Number(v) => write!(f, "{}", v),
            BodyType::String(v) => write!(f, "{}", v),
            BodyType::HashMap(v) => write!(f, "{}", serde_json::to_string_pretty(v).unwrap()),
            BodyType::Array(v) => write!(f, "{}", serde_json::to_string_pretty(v).unwrap()),
            BodyType::Null => write!(f, "null"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Expect {
    pub status: u16,
    pub body: BodyType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Callback {
    name: String,
    #[serde(flatten)]
    method: Method,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub method_type: HttpMethod,
    pub description: Option<String>,
    pub endpoint: String,
    pub body: Option<BodyType>,
    pub headers: Option<HashMap<String, String>>,
    pub expect: Expect,
    pub before_each: Option<Vec<String>>,
    pub after_each: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub base_url: String,
    pub callbacks: Option<Vec<Callback>>,
    pub timeout: u64,
    pub methods: Vec<Method>,
}
