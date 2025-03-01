use macroquad::{math::Vec2, texture::Texture2D};

use super::{sprite::Sprite, transform::Transform};

pub struct TurnMarker {
    pub visible: bool,
    pub transform: Transform,
    sprite: Sprite,
}

impl TurnMarker {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            visible: true,
            transform: Transform::new(),
            sprite: Sprite::new(texture),
        }
    }

    pub fn set_translation(&mut self, translation: Vec2) {
        self.transform.translation = translation;
    }

    pub fn draw(&mut self) {
        if self.visible {
            self.sprite.draw(&self.transform);
        }
    }
}
