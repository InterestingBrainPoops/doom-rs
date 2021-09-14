macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}
use vek::{num_traits::clamp, Vec2};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    corner1: Vec2<f64>,
    corner2: Vec2<f64>,
    corner3: Vec2<f64>,
}
impl Triangle {
    // More details for this at :
    // https://iquilezles.org/www/articles/distfunctions2d/distfunctions2d.htm
    // Specifically the generic triangle.
    pub fn signed_distance(&self, point: Vec2<f64>) -> f64 {
        let e1 = self.corner1 - self.corner2;
        let e2 = self.corner3 - self.corner2;
        let e3 = self.corner1 - self.corner3;
        let v1 = point - self.corner1;
        let v2 = point - self.corner2;
        let v3 = point - self.corner3;
        let pq1 = v1 - e1 * clamp(v1.dot(e1) / e1.dot(e1), 0.0, 1.0);
        let pq2 = v2 - e2 * clamp(v2.dot(e2) / e2.dot(e2), 0.0, 1.0);
        let pq3 = v3 - e3 * clamp(v3.dot(e3) / e3.dot(e3), 0.0, 1.0);
        let s = Self::sign(e1.x * e3.y - e1.y * e3.x);
        let d1 = Vec2 {
            x: pq1.dot(pq1),
            y: s * (v1.x * e1.y - v1.y * e1.x),
        };
        let d2 = Vec2 {
            x: pq2.dot(pq2),
            y: s * (v2.x * e2.y - v2.y * e2.x),
        };
        let d3 = Vec2 {
            x: pq3.dot(pq3),
            y: s * (v3.x * e3.y - v3.y * e3.x),
        };
        let d = Self::min(d1, Self::min(d2, d3));
        return -d.x.sqrt() * Self::sign(d.y);
    }

    fn min(v1: Vec2<f64>, v2: Vec2<f64>) -> Vec2<f64> {
        Vec2 {
            x: ternary!(v1.x < v2.x, v1.x, v2.x),
            y: ternary!(v1.y < v2.y, v1.y, v2.y),
        }
    }

    fn sign(value: f64) -> f64 {
        if value < 0.0 {
            return -1.0;
        } else if value > 0.0 {
            return 1.0;
        } else if value == 0.0 {
            return 0.0;
        } else {
            panic!("Something strange happened");
        }
    }
}
