use macroquad::{math::Vec2, texture::Texture2D};

use crate::view::transform::Transform;

use super::{sprite::Sprite, view_entity::ViewEntity};


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
}

impl ViewEntity for TurnMarker {
    fn draw(&mut self, _parent_transform: &Transform) {
        if self.visible {
            self.sprite.draw(&self.transform);
        }
    }
}
