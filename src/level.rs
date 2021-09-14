use serde::{Deserialize, Serialize};

use crate::triangle::Triangle;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Level {
    objects: Vec<Triangle>,
    width: u64,
    height: u64,
}
