use macroquad::prelude::*;

use super::transform::Transform;

/// A sprite (texture) component.
pub struct Sprite {
    pub texture: Texture2D,
    pub size: Vec2,
    pub anchor: Vec2,
    pub color: Color,
}

impl Sprite {
    pub fn new(texture: Texture2D) -> Self {
        let size = vec2(texture.width(), texture.height());
        Sprite::new_with_size(texture, size)
    }

    pub fn new_with_size(texture: Texture2D, size: Vec2) -> Self {
        Self {
            texture,
            size,
            anchor: vec2(0.5, 0.5),
            color: WHITE,
        }
    }

    pub fn new_with_size_mult(texture: Texture2D, size_mult: f32) -> Self {
        let size = vec2(texture.width() * size_mult, texture.height() * size_mult);
        Sprite::new_with_size(texture, size)
    }

    #[allow(unused)]
    pub fn set_size_from_multiplier(&mut self, multiplier: f32) {
        self.size = vec2(self.texture.width() * multiplier, self.texture.height() * multiplier);
    }

    pub fn draw(&mut self, transform: &Transform) {
        let (mut trans, rot, scale) = transform.trans_rot_scale();
        trans -= self.size * self.anchor;

        let params = DrawTextureParams {
            dest_size: Some(self.size * scale),
            rotation: rot,
            ..Default::default()
        };

        draw_texture_ex(&self.texture, trans.x, trans.y, self.color, params);
    }
}
