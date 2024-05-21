mod contract_reader;
mod fetch;

use std::{collections::HashMap, process::exit};

use contract_reader::{
    contract_structure::{Contract, Expect, HttpMethod},
    reader,
};
use fetch::fetch;
use serde_json::Result;

use crate::contract_reader::contract_structure::BodyType;

fn compare_and_print_diff<T: std::fmt::Display + PartialEq>(
    expected: &T,
    actual: &T,
    url: &str,
    method: &HttpMethod,
    description: &str,
) {
    if expected != actual {
        println!("Difference found in {}:\n", description);
        println!("Testing url: {}", url);
        println!("Testing with method: {:?}", method);
        println!("\nExpected: {}\nActual: {}", expected, actual);
        exit(1);
    }
}

fn main() -> Result<()> {
    let file_path = "./examples/simple.json";

    let content = reader(file_path.to_string());
    let contract: Contract = serde_json::from_str(&content)?;

    for method in contract.methods.iter() {
        let expect: &Expect = &method.expect;
        let url = [contract.base_url.clone(), method.endpoint.clone()].join("");

        let headers: HashMap<String, String> = match &method.headers {
            Some(v) => v.clone(),
            None => HashMap::new(),
        };

        let body: BodyType = match &method.body {
            Some(v) => v.clone(),
            None => BodyType::Null,
        };

        let result = fetch(
            &url,
            &method.method_type,
            &headers,
            &body,
            &contract.timeout,
        );

        compare_and_print_diff(
            &expect.status,
            &result.status,
            &url,
            &method.method_type,
            "status",
        );
        compare_and_print_diff(
            &expect.body,
            &result.body,
            &url,
            &method.method_type,
            "body",
        );
    }

    Ok(())
}
