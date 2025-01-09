use macroquad::{math::Vec2, texture::load_texture};

use crate::card::CardSuit;

use super::{imager::Imager, transform::Transform};

pub struct TrumpMarker {
    pub visible: bool,
    transform: Transform,
    image: Option<Imager>,
}

impl TrumpMarker {
    pub fn new(position: Vec2) -> Self {
        Self {
            visible: true,
            transform: Transform::new(position, 0.0),
            image: None,
        }
    }

    pub async fn set_suit(&mut self, suit: Option<CardSuit>) {
        if let Some(suit) = suit {
            let tex = match suit {
                CardSuit::Club => load_texture("src/assets/club.png").await.unwrap(),
                CardSuit::Diamond => load_texture("src/assets/diamond.png").await.unwrap(),
                CardSuit::Heart => load_texture("src/assets/heart.png").await.unwrap(),
                CardSuit::Spade => load_texture("src/assets/spade.png").await.unwrap(),
                CardSuit::Joker => todo!(),
            };
            self.image = Some(Imager::new(tex, 0.3333, true));
        } else {
            self.image = None;
        }
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }

        if let Some(image) = &mut self.image {
            image.draw(&self.transform, None);
        }
    }
}
