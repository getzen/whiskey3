use macroquad::prelude::*;

use crate::view::transform::Transform;

use super::view_entity::ViewEntity;

/// A sprite (texture) component.
pub struct Sprite {
    pub transform: Transform,
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
            transform: Transform::new(),
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
}

impl ViewEntity for Sprite {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw(&mut self, parent_transform: &Transform) {
        let transform = *parent_transform * self.transform;
        let (mut pos, rot, scale) = transform.trans_rot_scale();
        pos -= self.size * self.anchor;

        let params = DrawTextureParams {
            dest_size: Some(self.size * scale),
            rotation: rot,
            ..Default::default()
        };

        draw_texture_ex(&self.texture, pos.x, pos.y, self.color, params);
    }
}
