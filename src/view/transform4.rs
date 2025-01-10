use macroquad::math::{Mat4, Quat, Vec2, Vec3};

/* If speed is a concern, consider caching the computed matrix in an Option<Mat4>
and converting the properties to private with getters/setters. When set, they
set the matrix to None so it will be recomputed. */

#[derive(Clone)]
pub struct Transform4 {
    pub position: Vec2,
    /// Radians
    pub rotation: f32,
    /// Scale affects size only, not position or rotation.
    pub scale: Vec2,
}

impl Default for Transform4 {
    fn default() -> Self {
        Transform4 {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform4 {
    #[allow(dead_code)]
    pub fn from_position(position: Vec2) -> Self {
        Transform4 {
            position,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn from_position_rotation(position: Vec2, rotation: f32) -> Self {
        Transform4 {
            position,
            rotation,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    /// Returns a new Transform created by multiplying own matrix by other.
    pub fn from_matrix(matrix: Mat4) -> Self {
        let (scale, rot, trans) = matrix.to_scale_rotation_translation();
        let (_, rotation) = rot.to_axis_angle();

        Self {
            position: Vec2::new(trans.x, trans.y),
            rotation,
            scale: Vec2::new(scale.x, scale.y),
        }
    }

    /// Returns a Mat4 calculated from the attributes. Macroquad uses Mat4 for the
    /// model matrix, eg: gl.push_model_matrix(matrix);
    pub fn matrix(&self, centering_size: Option<Vec2>) -> Mat4 {
        let scale = Vec3::new(self.scale.x, self.scale.y, 1.0);
        let translation = Vec3::new(self.position.x, self.position.y, 0.0);
        let rotation = Quat::from_rotation_z(self.rotation);
        let matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        match centering_size {
            Some(size) => {
                matrix * Mat4::from_translation(Vec3::new(-size.x * 0.5, -size.y * 0.5, 0.0))
            }
            None => matrix,
        }
    }

    pub fn contains_point(&self, screen_pt: Vec2, size: Vec2, centered: bool) -> bool {
        let point3 = Vec3::new(screen_pt.x, screen_pt.y, 0.0);
        let matrix = match centered {
            true => self.matrix(Some(size)),
            false => self.matrix(None),
        };
        let world_pt = matrix.inverse().transform_point3(point3);
        world_pt.x >= 0.0 && world_pt.y >= 0.0 && world_pt.x <= size.x && world_pt.y <= size.y
    }
}
