mod contract_reader;

use contract_reader::contract_structure::Contract;
use serde_json::Result;

fn main() -> Result<()> {
    let example = r#"
        {
            "base_url": "http://localhost:8000/",
            "timeout": 10000,
            "methods": [
                {
                    "method_type": "GET",
                    "body": { "name": "walefy" },
                    "expect": {
                        "status": 500,
                        "body": "feito"
                    }
                }
            ]
        }
    "#;

    let contract_example: Contract = serde_json::from_str(&example)?;

    println!("{:#?}", contract_example.methods[0].body);
    Ok(())
}
