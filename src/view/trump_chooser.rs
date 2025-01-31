use std::sync::mpsc::Sender;

use macroquad::{
    math::{vec2, Vec2},
    texture::load_texture,
};

use crate::{card::Suit, game::PlayerAction};

use super::{button_shaded::ButtonShaded, transform::Transform};

pub struct TrumpChooser {
    pub visible: bool,
    transform: Transform,
    club_button: ButtonShaded,
    diamond_button: ButtonShaded,
    heart_button: ButtonShaded,
    spade_button: ButtonShaded,
}

impl TrumpChooser {
    pub async fn new(position: Vec2, sender: Sender<PlayerAction>) -> Self {
        let tex_mult = 0.3333;

        let tex = load_texture("src/assets/club.png").await.unwrap();
        let mut club_button = ButtonShaded::new(position + vec2(-90.0, 0.), tex, tex_mult);
        club_button.sender = Some(sender.clone());
        club_button.action = Some(PlayerAction::ChooseTrump(Suit::Club));

        let tex = load_texture("src/assets/diamond.png").await.unwrap();
        let mut diamond_button = ButtonShaded::new(position + vec2(-30.0, 0.), tex, tex_mult);
        diamond_button.sender = Some(sender.clone());
        diamond_button.action = Some(PlayerAction::ChooseTrump(Suit::Diamond));

        let tex = load_texture("src/assets/heart.png").await.unwrap();
        let mut heart_button = ButtonShaded::new(position + vec2(30.0, 0.), tex, tex_mult);
        heart_button.sender = Some(sender.clone());
        heart_button.action = Some(PlayerAction::ChooseTrump(Suit::Heart));

        let tex = load_texture("src/assets/spade.png").await.unwrap();
        let mut spade_button = ButtonShaded::new(position + vec2(90.0, 0.), tex, tex_mult);
        spade_button.sender = Some(sender.clone());
        spade_button.action = Some(PlayerAction::ChooseTrump(Suit::Spade));

        Self {
            visible: false,
            transform: Transform::from_translation(position),
            club_button,
            diamond_button,
            heart_button,
            spade_button,
        }
    }

    /// Returns true if event found.
    pub fn process_events(&mut self, parent_transform: Option<&Transform>, mouse_pos: Vec2) -> bool {
        if !self.visible {
            return false;
        }

        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };

        let mouse_over0 = self.club_button.process_events(Some(transform), mouse_pos);
        let mouse_over1 = self.diamond_button.process_events(Some(transform), mouse_pos);
        let mouse_over2 = self.heart_button.process_events(Some(transform), mouse_pos);
        let mouse_over3 = self.spade_button.process_events(Some(transform), mouse_pos);

        mouse_over0 || mouse_over1 || mouse_over2 || mouse_over3
    }

    pub fn draw(&mut self, parent_transform: Option<&Transform>) {
        if !self.visible {
            return;
        }

        let transform = match parent_transform {
            Some(parent) => &Transform::combine(parent, &self.transform),
            None => &self.transform,
        };

        self.club_button.draw(Some(transform));
        self.diamond_button.draw(Some(transform));
        self.heart_button.draw(Some(&transform));
        self.spade_button.draw(Some(&transform));
    }
}
