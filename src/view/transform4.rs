use macroquad::math::{Mat4, Quat, Vec2, Vec3};

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
    pub fn combine_matrix(&self, other: Mat4) -> Self {
        let combined = self.matrix() * other;
        let (scale, rot, trans) = combined.to_scale_rotation_translation();
        let (_, rotation) = rot.to_axis_angle();

        Self {
            position: Vec2::new(trans.x, trans.y),
            rotation,
            scale: Vec2::new(scale.x, scale.y),
        }
    }

    /// Returns a Mat4 calculated from the attributes. Macroquad uses Mat4 for the
    /// model matrix, eg: gl.push_model_matrix(matrix);
    pub fn matrix(&self) -> Mat4 {
        let scale = Vec3::new(self.scale.x, self.scale.y, 1.0);
        let translation = Vec3::new(self.position.x, self.position.y, 0.0);
        let rotation = Quat::from_rotation_z(self.rotation);
        Mat4::from_scale_rotation_translation(scale, rotation, translation)
    }

    /// Returns a Mat4 calculated from the attributes.
    pub fn matrix_centered(&self, centering_size: Vec2) -> Mat4 {
        self.matrix()
            * Mat4::from_translation(Vec3::new(
                -centering_size.x * 0.5,
                -centering_size.y * 0.5,
                0.0,
            ))
    }

    pub fn contains_point(&self, screen_pt: Vec2, size: Vec2, centered: bool) -> bool {
        let point3 = Vec3::new(screen_pt.x, screen_pt.y, 0.0);
        let matrix = match centered {
            true => self.matrix_centered(size),
            false => self.matrix(),
        };
        let world_pt = matrix.inverse().transform_point3(point3);
        world_pt.x >= 0.0 && world_pt.y >= 0.0 && world_pt.x <= size.x && world_pt.y <= size.y
    }

    // pub fn matrix_contains_point(matrix: Mat4, screen_pt: Vec2, size: Vec2) -> bool {
    //     let point3 = Vec3::new(screen_pt.x, screen_pt.y, 0.0);
    //     let world_pt = matrix.inverse().transform_point3(point3);
    //     world_pt.x >= 0.0 && world_pt.y >= 0.0 && world_pt.x <= size.x && world_pt.y <= size.y
    // }

    // KEEP THIS for the math, if nothing else.
    // Returns true if the given point is contained in the translated, rotated,
    // scaled, sized and possibly centered rectangle or false if not.
    // pub fn contains_point(&self, point: (i32, i32)) -> bool {
    //     // Get the net test point relative to the transform's position.
    //     let net_x = point.0 as f32 - self.position.0;
    //     let net_y = point.1 as f32 - self.position.1;

    //     // Rotate the point clockwise. (Notan and Macroquad both rotate clockwise.)
    //     let theta = self.rotation;
    //     let rot_x = net_x * f32::cos(theta) + net_y * f32::sin(theta);
    //     let rot_y = -net_x * f32::sin(theta) + net_y * f32::cos(theta);

    //     // See if the rotated point is in the unrotated size rectangle.
    //     let (mut w, mut h) = self.size;
    //     w *= self.scale.0;
    //     h *= self.scale.1;

    //     if self.centered {
    //         rot_x.abs() <= w * 0.5 && rot_y.abs() <= h * 0.5
    //     } else {
    //         rot_x >= 0.0 && rot_x <= w && rot_y >= 0.0 && rot_y <= h
    //     }
    // }
}
