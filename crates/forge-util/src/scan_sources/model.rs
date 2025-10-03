use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Default, Serialize)]
pub struct DirNode {
    pub files: Vec<String>,
    pub dirs: BTreeMap<String, DirNode>,
}

pub type CrateMap = BTreeMap<String, DirNode>;
