use crate::{coordinate::Coordinate, vector::Vector};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    position: Coordinate,
    camera_vector: Vector,
    health: u8,
}
