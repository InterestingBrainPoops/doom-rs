use crate::edge::Edge;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
    edges: Vec<Edge>,
}
