use macroquad::{math::Vec2, texture::Texture2D};

use super::{sprite::Sprite, transform::Transform};

pub struct TurnMarker {
    pub visible: bool,
    pub transform: Transform,
    sprite: Sprite,
}

impl TurnMarker {
    pub fn new(texture: Texture2D, size: Vec2) -> Self {
        Self {
            visible: true,
            transform: Transform::new(),
            sprite: Sprite::new_with_size(texture, size),
        }
    }

    pub fn draw(&mut self) {
        if self.visible {
            self.sprite.draw(&self.transform);
        }
    }
}
