use macroquad::prelude::*;

use crate::view::trans::Trans;

pub struct Imager {
    pub visible: bool,
    pub transform: Trans,
    pub texture: Texture2D,
    pub color: Color,
    pub z_order: usize,
}

impl Imager {
    pub fn new(texture: Texture2D, size_multiplier: f32, centered: bool) -> Self {
        let size = vec2(texture.width() * size_multiplier, texture.height() * size_multiplier);
        let mut transform = Trans::new();
        match centered {
            true => transform.center_with_size(size),
            false => transform.size = size,
        }

        Self {
            visible: true,
            transform,
            texture,
            color: WHITE,
            z_order: 0,
        }
    }

    pub fn draw(&self, parent_transform: Option<&Trans>) {
        if !self.visible {
            return;
        }

        let transform = match parent_transform {
            Some(parent) => &Trans::combine(parent, &self.transform),
            None => &self.transform,
        };

        let (pos, rot) = transform.drawable_position_rotation();

        let params = DrawTextureParams {
            dest_size: Some(transform.size),
            rotation: rot,
            ..Default::default()
        };

        draw_texture_ex(&self.texture, pos.x, pos.y, self.color, params);
    }
}
