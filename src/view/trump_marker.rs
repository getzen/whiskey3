use macroquad::{math::Vec2, texture::load_texture};

use crate::card::Suit;

use super::{transform::Transform, view_entity::{sprite::Sprite, view_entity::ViewEntity}};

pub struct TrumpMarker {
    pub visible: bool,
    transform: Transform,
    sprite: Option<Sprite>,
}

impl TrumpMarker {
    pub fn new(position: Vec2) -> Self {
        Self {
            visible: true,
            transform: Transform::from_translation(position),
            sprite: None,
        }
    }

    pub async fn set_suit(&mut self, suit: Option<Suit>) {
        if let Some(suit) = suit {
            let tex = match suit {
                Suit::Club => load_texture("src/assets/club.png").await.unwrap(),
                Suit::Diamond => load_texture("src/assets/diamond.png").await.unwrap(),
                Suit::Heart => load_texture("src/assets/heart.png").await.unwrap(),
                Suit::Spade => load_texture("src/assets/spade.png").await.unwrap(),
                Suit::Joker => load_texture("src/assets/joker.png").await.unwrap(),
            };
            let mut sprite = Sprite::new(tex);
            sprite.set_size_from_multiplier(0.3333);
            self.sprite = Some(sprite);
        } else {
            self.sprite = None;
        }
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }

        if let Some(sprite) = &mut self.sprite {
            sprite.draw(&self.transform);
        }
    }
}
