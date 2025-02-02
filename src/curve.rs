use bevy::prelude::*;

pub struct Curve {
    pub points: Vec<Vec3>,
    pub points_u: Vec<f32>,
    pub length: f32,
}

impl Curve {
    pub fn from(points: Vec<Vec3>) -> Self {
        let length = points
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                points
                    .get(idx + 1)
                    .map(|next_p| (*next_p - *p).length())
                    .unwrap_or(0.0)
            })
            .sum();

        let mut length_traveled = 0.0;
        let mut points_u = Vec::new();

        for (idx, pt) in points.iter().enumerate() {
            points_u.push(length_traveled / length);
            if let Some(next_pt) = points.get(idx + 1) {
                length_traveled += (*next_pt - *pt).length();
            }
        }

        Self {
            points,
            points_u,
            length,
        }
    }

    // Curve segment is defined by start_point_index and end_point_index
    fn get_curve_segment_from_u(&self, u: f32) -> (usize, usize) {
        if u == 1.0 {
            return (self.points.len() - 2, self.points.len() - 1);
        } else if u == 0.0 {
            return (0, 1);
        }

        for (i, pt_u) in self.points_u.iter().enumerate() {
            if u <= *pt_u {
                return (i - 1, i);
            }
        }

        unreachable!()
    }

    pub fn get_pos_at_u(&self, u: f32) -> Vec3 {
        assert!(u <= 1.0 && u >= 0.0, "u is in incorrect range");

        let (idx1, idx2) = self.get_curve_segment_from_u(u);

        let dir = self.points[idx2] - self.points[idx1];
        let u_range = (self.points_u[idx1], self.points_u[idx2]);

        let mag = (u - u_range.0) / (u_range.1 - u_range.0);

        self.points[idx1] + dir * mag
    }

    pub fn get_tangent_at_u(&self, u: f32) -> Vec3 {
        assert!(u <= 1.0 && u >= 0.0, "u is in incorrect range");

        let (idx1, idx2) = self.get_curve_segment_from_u(u);

        (self.points[idx2] - self.points[idx1]).normalize()
    }
}
