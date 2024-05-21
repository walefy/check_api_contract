use serde::{Deserialize, Serialize};

use crate::contract_reader::contract_structure::BodyType;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: BodyType,
    // adicionar headers tbm
}
