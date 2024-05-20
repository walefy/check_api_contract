mod contract_reader;

use contract_reader::{contract_structure::Contract, reader};
use serde_json::Result;

fn main() -> Result<()> {
    let file_path = "./examples/simple.json";

    let content = reader(file_path.to_string());
    let contract: Contract = serde_json::from_str(&content)?;

    println!("{:#?}", contract.methods[0].expect);

    Ok(())
}
