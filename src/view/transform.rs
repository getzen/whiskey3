use std::ops::{Mul, MulAssign};

use macroquad::math::{Affine2, Mat4, Quat, Vec2, Vec3, vec2};

#[derive(Copy, Clone)]
pub struct Transform {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform {
    #[allow(unused)]
    pub fn new() -> Self {
        Transform::default()
    }

    #[allow(unused)]
    /// Create from translation x, y.
    pub fn from_x_y(x: f32, y: f32) -> Self {
        Self {
            translation: vec2(x, y),
            ..Default::default()
        }
    }

    #[allow(unused)]
    /// Create from translation x, y, and rotation.
    pub fn from_x_y_r(x: f32, y: f32, rotation: f32) -> Self {
        Self {
            translation: vec2(x, y),
            rotation,
            ..Default::default()
        }
    }

    #[allow(unused)]
    /// Create from translation x, y, rotation, and scale x, y.
    pub fn from_x_y_r_scale(x: f32, y: f32, rotation: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            translation: vec2(x, y),
            rotation,
            scale: vec2(scale_x, scale_y),
        }
    }

    #[allow(unused)]
    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    #[allow(unused)]
    /// Creates a new Transform by multiplying self by another.
    /// Unlike the * operator, this works with references.
    pub fn from_multiplying(lhs: &Transform, rhs: &Transform) -> Self {
        let combined = lhs.affine() * rhs.affine();
        let (scale, rotation, translation) = combined.to_scale_angle_translation();
        Self {
            translation,
            rotation,
            scale,
        }
    }

    /// Returns an Affine2 created from properties.
    pub fn affine(&self) -> Affine2 {
        Affine2::from_scale_angle_translation(self.scale, self.rotation, self.translation)
    }

    /// Convenience method to return translation, rotation, and scale in a compact way.
    pub fn trans_rot_scale(&self) -> (Vec2, f32, Vec2) {
        (self.translation, self.rotation, self.scale)
    }

    /// Returns a Mat4 created from self properties. Macroquad and OpenGL use Mat4
    /// for the model matrix, eg:
    /// let gl = unsafe { get_internal_gl().quad_gl };
    /// gl.push_model_matrix(matrix);
    /// draw...
    /// gl.pop_model_matrix();
    #[allow(unused)]
    pub fn matrix(&self) -> Mat4 {
        let scale = Vec3::new(self.scale.x, self.scale.y, 1.0);
        let translation = Vec3::new(self.translation.x, self.translation.y, 0.0);
        let rotation = Quat::from_rotation_z(self.rotation);
        let m = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        let m = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        m * m
    }
}

/// This only works with owned Transforms, not borrowed.
impl Mul for Transform {
    type Output = Transform;

    fn mul(self, rhs: Self) -> Self::Output {
        let combined = self.affine() * rhs.affine();
        let (scale, rotation, translation) = combined.to_scale_angle_translation();
        Self {
            translation,
            rotation,
            scale,
        }
    }
}

impl MulAssign for Transform {
    fn mul_assign(&mut self, rhs: Self) {
        let combined = self.affine() * rhs.affine();
        let (scale, rotation, translation) = combined.to_scale_angle_translation();
        *self = Self {
            translation,
            rotation,
            scale,
        }
    }
}
