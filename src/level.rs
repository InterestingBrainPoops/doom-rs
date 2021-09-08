use serde::{Deserialize, Serialize};

use crate::object::Object;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Level {
    objects: Vec<Object>,
    width: u64,
    height: u64,
}
