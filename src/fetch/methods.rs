use std::collections::HashMap;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::contract_reader::contract_structure::BodyType;

use super::response::Response;

pub fn get(
    url: &String,
    headers: &HashMap<String, String>,
    body: &BodyType,
    client: &Client,
) -> Response {
    let body_str = body.to_string().unwrap_or("null".to_string());
    let mut l_headers: HeaderMap = HeaderMap::new();

    for (k, v) in headers.iter() {
        l_headers.insert(
            HeaderName::from_bytes(k.as_bytes()).unwrap(),
            HeaderValue::from_bytes(v.as_bytes()).unwrap(),
        );
    }

    let res = client
        .get(url)
        .body(body_str)
        .headers(l_headers)
        .send()
        .unwrap();

    Response {
        status: res.status().as_u16(),
        body: serde_json::from_str(&res.text().unwrap()).unwrap(),
    }
}
