use super::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Autotag {
    pub program: String,
    pub tag: String,
}
