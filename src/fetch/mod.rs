mod methods;
mod response;

use reqwest::blocking::Client;
use std::collections::HashMap;

use crate::contract_reader::contract_structure::{BodyType, HttpMethod};

use self::methods::build_request;
use self::response::Response;

pub fn fetch(
    url: String,
    method: &HttpMethod,
    headers: &HashMap<String, String>,
    body: &BodyType,
    timeout: &u64,
) -> Response {
    let client = Client::new();
    build_request(method, &url, headers, body, timeout, &client)
}
