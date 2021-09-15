use serde::{Deserialize, Serialize};

use crate::triangle::Triangle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub objects: Vec<Triangle>,
    width: u64,
    height: u64,
}
