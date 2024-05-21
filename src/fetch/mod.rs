mod response;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::process::exit;
use std::time::Duration;

use crate::contract_reader::contract_structure::{BodyType, HttpMethod};

use self::response::Response;

pub fn fetch(
    url: &str,
    method: &HttpMethod,
    headers: &HashMap<String, String>,
    body: &BodyType,
    timeout: &u64,
) -> Response {
    let client = Client::new();

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

    let res_result = request
        .body(body_str)
        .headers(l_headers)
        .timeout(Duration::from_millis(timeout.to_owned()))
        .send();

    let res = match res_result {
        Ok(v) => v,
        Err(_) => {
            println!("Connection error with url: {url}");
            exit(1);
        }
    };

    Response {
        status: res.status().as_u16(),
        body: serde_json::from_str(&res.text().unwrap()).unwrap(),
    }
}
