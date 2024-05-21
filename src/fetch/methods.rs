use std::{collections::HashMap, time::Duration};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::contract_reader::contract_structure::{BodyType, HttpMethod};

use super::response::Response;

pub fn build_request(
    method: &HttpMethod,
    url: &str,
    headers: &HashMap<String, String>,
    body: &BodyType,
    timeout: &u64,
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

    let request = match method {
        HttpMethod::Get => client.get(url),
        HttpMethod::Post => client.post(url),
    };

    let res = request
        .body(body_str)
        .headers(l_headers)
        .timeout(Duration::from_millis(timeout.to_owned()))
        .send()
        .unwrap();

    Response {
        status: res.status().as_u16(),
        body: serde_json::from_str(&res.text().unwrap()).unwrap(),
    }
}
