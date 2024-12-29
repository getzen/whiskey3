use macroquad::prelude::*;

use crate::view::transform::Transform;

pub struct Imager {
    pub visible: bool,
    pub centered: bool,
    pub texture: Texture2D,
    pub tex_size_multiplier: f32,
    pub z_order: usize,
    params: DrawTextureParams,
}

impl Imager {
    pub fn new(texture: Texture2D, tex_size_multiplier: f32, centered: bool) -> Self {
        let sprite = Self {
            visible: true,
            centered,
            texture,
            tex_size_multiplier,
            z_order: 0,
            params: DrawTextureParams::default(),
        };
        sprite
    }

    pub fn draw_size(&self) -> Vec2 {
        vec2(
            self.texture.width() * self.tex_size_multiplier,
            self.texture.height() * self.tex_size_multiplier,
        )
    }

    pub fn draw(&mut self, transform: &Transform, color: Option<Color>) {
        if !self.visible {
            return;
        }

        let size = self.draw_size();

        let (mut pos, rot) = transform.combined_pos_rot();

        if self.centered {
            pos.x -= size.x / 2.0;
            pos.y -= size.y / 2.0;
        }

        self.params.rotation = rot;
        self.params.dest_size = Some(Vec2::new(size.x, size.y));

        let draw_color = color.unwrap_or(WHITE);
        draw_texture_ex(&self.texture, pos.x, pos.y, draw_color, self.params.clone());
    }
}
