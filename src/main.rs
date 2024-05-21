mod contract_reader;

use contract_reader::{
    contract_structure::{BodyType, Contract, Expect},
    reader,
};
use serde_json::Result;

fn main() -> Result<()> {
    let file_path = "./examples/simple.json";

    let content = reader(file_path.to_string());
    let contract: Contract = serde_json::from_str(&content)?;

    let example_result_status: u16 = 500;
    let example_result_body = BodyType::String("feito".to_string());

    for method in contract.methods.iter() {
        let expect: &Expect = &method.expect;

        assert!(expect.status == example_result_status);
        assert!(expect.body == example_result_body);
    }

    Ok(())
}
