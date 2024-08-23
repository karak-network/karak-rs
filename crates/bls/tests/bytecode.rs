use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecode {
    pub object: String,
    pub link_references: BTreeMap<String, BTreeMap<String, Vec<Offset>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offset {
    pub start: usize,
    pub length: usize,
}
