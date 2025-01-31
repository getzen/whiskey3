use macroquad::math::{Affine2, Mat4, Quat, Vec2, Vec3};

#[derive(Clone)]
pub struct Transform {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    pub size: Vec2,
    pub offset: Vec2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            size: Vec2::ZERO,
            offset: Vec2::ZERO,
        }
    }
}

impl Transform {
    #[allow(unused)]
    pub fn new() -> Self {
        Transform::default()
    }

    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn from_translation_size_centered(translation: Vec2, size: Vec2, centered: bool) -> Self {
        let offset = match centered {
            true => Vec2::new(0.5, 0.5),
            false => Vec2::ZERO,
        };

        Self {
            translation,
            size,
            offset,
            ..Default::default()
        }
    }

    /// Creates a new Transform by multiplying the parent and child affines to
    /// get the translation, rotation, and scale. Includes the child's size and offset.
    pub fn combine(parent: &Transform, child: &Transform) -> Self {
        let combined = parent.affine() * child.affine();
        let (scale, rotation, translation) = combined.to_scale_angle_translation();
        Self {
            translation,
            rotation,
            scale,
            size: child.size,
            offset: child.offset,
        }
    }

    pub fn offset_translation(&self) -> Vec2 {
        Vec2::new(-self.size.x * self.offset.x, -self.size.y * self.offset.y)
    }

    /// A convenience method to center the Transform using the given size.
    #[allow(unused)]
    pub fn center_with_size(&mut self, size: Vec2) {
        self.size = size;
        self.offset = Vec2::new(0.5, 0.5);
    }

    /// Returns an Affine2 created from self properties, ignoring offet and size.
    pub fn affine(&self) -> Affine2 {
        Affine2::from_scale_angle_translation(self.scale, self.rotation, self.translation)
    }

    /// Returns an Affine2 created from self properties and offset using offset and size.
    pub fn offset_affine(&self) -> Affine2 {
        Affine2::from_scale_angle_translation(
            self.scale,
            self.rotation,
            self.translation + self.offset_translation(),
        )
        //self.affine() * Affine2::from_translation(offset_trans)
    }

    /// Returns the drawable position and rotation based on the affine and including any offset and size.
    pub fn drawable_position_rotation(&self) -> (Vec2, f32) {
        let (_scale, angle, translation) = self.offset_affine().to_scale_angle_translation();
        (translation, angle)
    }

    /// Returns true if the given screen pt lies within the translated, rotated, and scaled
    /// size boundaries.
    pub fn contains_point(&self, screen_pt: Vec2) -> bool {
        let mut world_pt = self.affine().inverse().transform_point2(screen_pt);
        world_pt -= self.offset_translation();
        world_pt.x >= 0.0
            && world_pt.y >= 0.0
            && world_pt.x <= self.size.x
            && world_pt.y <= self.size.y
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
        let matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        let off_tran = self.offset_translation();
        matrix * Mat4::from_translation(Vec3::new(off_tran.x, off_tran.y, 0.0))
    }
}
