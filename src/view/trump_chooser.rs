use std::sync::mpsc::Sender;

use macroquad::{
    math::{vec2, Vec2},
    texture::load_texture,
};

use crate::{card::CardSuit, game::PlayerAction};

use super::{button_shaded::ButtonShaded, transform::Transform};

pub struct TrumpChooser {
    pub visible: bool,
    size: Vec2,
    transform: Transform,
    club_button: ButtonShaded,
    diamond_button: ButtonShaded,
    heart_button: ButtonShaded,
    spade_button: ButtonShaded,
    sender: Sender<PlayerAction>,
}

impl TrumpChooser {
    pub async fn new(position: Vec2, sender: Sender<PlayerAction>) -> Self {
        let tex_mult = 0.3333;

        let tex = load_texture("src/assets/club.png").await.unwrap();
        let club_button = ButtonShaded::new(0, position + vec2(-90.0, 0.), tex, tex_mult);

        let tex = load_texture("src/assets/diamond.png").await.unwrap();
        let diamond_button = ButtonShaded::new(0, position + vec2(-30.0, 0.), tex, tex_mult);

        let tex = load_texture("src/assets/heart.png").await.unwrap();
        let heart_button = ButtonShaded::new(0, position + vec2(30.0, 0.), tex, tex_mult);

        let tex = load_texture("src/assets/spade.png").await.unwrap();
        let spade_button = ButtonShaded::new(0, position + vec2(90.0, 0.), tex, tex_mult);

        Self {
            visible: false,
            size: vec2(250.0, 100.0),
            transform: Transform::new(position, 0.0),
            club_button,
            diamond_button,
            heart_button,
            spade_button,
            sender,
        }
    }

    /// Returns true if event found.
    pub fn process_events(&mut self, mouse_pos: &Vec2) -> bool {
        if !self.visible {
            return false;
        }

        if self.club_button.process_events(mouse_pos) {
            self.sender
                .send(PlayerAction::ChooseTrump(CardSuit::Club))
                .expect("Send error");
            return true;
        }

        if self.diamond_button.process_events(mouse_pos) {
            self.sender
                .send(PlayerAction::ChooseTrump(CardSuit::Diamond))
                .expect("Send error");
            return true;
        }

        if self.heart_button.process_events(mouse_pos) {
            self.sender
                .send(PlayerAction::ChooseTrump(CardSuit::Heart))
                .expect("Send error");
            return true;
        }

        if self.spade_button.process_events(mouse_pos) {
            self.sender
                .send(PlayerAction::ChooseTrump(CardSuit::Spade))
                .expect("Send error");
        }
        false
    }

    pub fn draw(&mut self) {
        if !self.visible {
            return;
        }

        let (mut pos, _rot) = self.transform.combined_pos_rot();
        pos.x -= self.size.x / 2.0;
        pos.y -= self.size.y / 2.0;

        //draw_rectangle(pos.x, pos.y, self.size.x, self.size.y, );
        //draw_rectangle_lines(pos.x, pos.y, self.size.x, self.size.y, 4.0, GREEN);

        self.club_button.draw();
        self.diamond_button.draw();
        self.heart_button.draw();
        self.spade_button.draw();
    }
}
