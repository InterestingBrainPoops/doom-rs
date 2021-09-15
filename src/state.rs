use vek::Vec2;

use crate::{level::Level, player::Player};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    level: Level,
    player: Player,
}

impl State {
    /// Basically renders the current scene by returning the over
    pub fn render(&self) -> [f64; 800] {
        let mut out = [0.0; 800];
        for (index, y) in out.iter_mut().enumerate() {
            let plane_offset = (index as f64 - 400.0) / 400.0;
            let plane_modifier = plane_offset * self.player.plane;
            let mut ray = plane_modifier + self.player.camera_vector + self.player.position;
            ray.normalize();
            let mut dist = Vec2::new(0.0, 0.0);
            for _ in 0..200 {
                let next_dist = self.get_shortest_sdf(self.player.position + ray * dist);
                if next_dist < 0.001 {
                    break;
                }
                dist += next_dist * ray;
            }
            let perp_wall_dist = dist.y / ray.y;
            *y = perp_wall_dist;
        }
        out
    }
    /// Return the shortest SDF to any triangle in the scene.
    fn get_shortest_sdf(&self, position: Vec2<f64>) -> f64 {
        let mut shortest = f64::MAX;
        for x in &self.level.objects {
            let dist = x.signed_distance(position);
            if dist < shortest {
                shortest = dist;
            }
        }
        shortest
    }
}
