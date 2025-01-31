use macroquad::math::{Mat3, Vec2};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: Vec2, rotation: f32) -> Self {
        Self { position, rotation }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
        }
    }

    /// Returns the x, y positions.
    pub fn combined_pos_rot(&self) -> (Vec2, f32) {
        (self.position, self.rotation)
    }

    pub fn centered_position(&self, size: Vec2) -> Vec2 {
        Vec2::new(
            self.position.x - size.x * 0.5,
            self.position.y - size.y * 0.5,
        )
    }

    /// Returns a matrix calculated from the attributes
    pub fn matrix(&self) -> Mat3 {
        let translation = Mat3::from_translation(Vec2::new(self.position.x, self.position.y));
        let rotation = Mat3::from_angle(self.rotation);
        translation * rotation
        //let scale = Mat3::from_scale(Vec2::new(self.scale.0, self.scale.1));
        //translation * rotation * scale
    }

    /// Returns the position as rotated by the given angle.
    fn rotated_position(&self, angle: f32) -> Vec2 {
        let angle_vec = Vec2::from_angle(angle);
        self.position.rotate(angle_vec)
    }
}
