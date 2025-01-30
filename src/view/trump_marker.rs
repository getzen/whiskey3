use macroquad::{math::Vec2, texture::load_texture};

use crate::card::Suit;

use super::{imager::Imager, trans::Trans};

pub struct TrumpMarker {
    pub visible: bool,
    transform: Trans,
    imager: Option<Imager>,
}

impl TrumpMarker {
    pub fn new(position: Vec2) -> Self {
        Self {
            visible: true,
            transform: Trans::from_translation(position),
            imager: None,
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
            self.imager = Some(Imager::new(tex, 0.3333, true));
        } else {
            self.imager = None;
        }
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }

        if let Some(image) = &mut self.imager {
            image.draw(Some(&self.transform));
        }
    }
}
