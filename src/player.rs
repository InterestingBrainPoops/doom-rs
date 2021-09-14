use vek::Vec2;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// Some constants:
// FOV will be 66 degrees.

pub struct Player {
    position: Vec2<f64>,
    camera_vector: Vec2<f64>,
    plane: Vec2<f64>,
    health: u8,
}
