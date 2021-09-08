use crate::coordinate::Coordinate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge {
    end: Coordinate,
    start: Coordinate,
}
