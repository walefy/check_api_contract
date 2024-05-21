mod methods;
mod response;

use reqwest::blocking::Client;
use std::collections::HashMap;

use crate::contract_reader::contract_structure::BodyType;

use self::methods::get;
use self::response::Response;

pub fn fetch(
    url: String,
    method: &str,
    headers: &HashMap<String, String>,
    body: &BodyType,
) -> Response {
    let client = Client::new();
    let mut strategy_methods: HashMap<&'static str, Box<dyn Fn() -> Response>> = HashMap::new();

    strategy_methods.insert("get", Box::new(|| get(&url, headers, body, &client)));

    match strategy_methods.get(method) {
        Some(func) => func(),
        None => panic!(),
    }
}
