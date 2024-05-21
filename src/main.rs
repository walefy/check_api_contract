mod contract_reader;
mod fetch;

use std::collections::HashMap;

use contract_reader::{
    contract_structure::{Contract, Expect},
    reader,
};
use fetch::fetch;
use serde_json::Result;

use crate::contract_reader::contract_structure::BodyType;

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

        let result = fetch(url, &method.method_type, &headers, &body, &contract.timeout);

        assert!(expect.status == result.status);
        assert!(expect.body == result.body);
    }

    Ok(())
}
