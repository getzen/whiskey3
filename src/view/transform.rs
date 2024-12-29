use macroquad::math::Vec2;

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: Vec2, rotation: f32) -> Self {
        Self {
            position,
            rotation,
        }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
        }
    }

    pub fn centered_position(&self, size: Vec2) -> Vec2 {
        Vec2::new(self.position.x - size.x * 0.5, self.position.y - size.y * 0.5)
    }
}
