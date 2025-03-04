use macroquad::{
    math::{Vec2, vec2},
    texture::load_texture,
};

use crate::{card::Suit, game::PlayerAction, view::transform::Transform};

use super::{button_shaded::ButtonShaded, view_entity::ViewEntity};

pub struct TrumpChooser {
    pub visible: bool,
    transform: Transform,
    club_button: ButtonShaded,
    diamond_button: ButtonShaded,
    heart_button: ButtonShaded,
    spade_button: ButtonShaded,
}

impl TrumpChooser {
    pub async fn new(position: Vec2) -> Self {
        let tex_mult = 0.3333;

        let club_tex = load_texture("src/assets/club.png").await.unwrap();
        let diamond_tex = load_texture("src/assets/diamond.png").await.unwrap();
        let heart_tex = load_texture("src/assets/heart.png").await.unwrap();
        let spade_tex = load_texture("src/assets/spade.png").await.unwrap();

        Self {
            visible: false,
            transform: Transform::from_translation(position),

            club_button: ButtonShaded::new(
                vec2(-90., 0.),
                club_tex,
                tex_mult,
                Some(PlayerAction::ChooseTrump(Suit::Club)),
            ),

            diamond_button: ButtonShaded::new(
                vec2(-30., 0.),
                diamond_tex,
                tex_mult,
                Some(PlayerAction::ChooseTrump(Suit::Diamond)),
            ),

            heart_button: ButtonShaded::new(
                vec2(30., 0.),
                heart_tex,
                tex_mult,
                Some(PlayerAction::ChooseTrump(Suit::Heart)),
            ),

            spade_button: ButtonShaded::new(
                vec2(90., 0.),
                spade_tex,
                tex_mult,
                Some(PlayerAction::ChooseTrump(Suit::Spade)),
            ),
        }
    }
}

impl ViewEntity for TrumpChooser {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn process_mouse(&mut self, mouse_pos: &Vec2, _parent_transform: &Transform) -> bool {
        if !self.visible {
            return false;
        }
        let mut mouse_over = self.club_button.process_mouse(&mouse_pos, &self.transform);
        mouse_over = mouse_over || self.diamond_button.process_mouse(&mouse_pos, &self.transform);
        mouse_over = mouse_over || self.heart_button.process_mouse(&mouse_pos, &self.transform);
        mouse_over = mouse_over || self.spade_button.process_mouse(&mouse_pos, &self.transform);
        mouse_over
    }

    fn draw(&mut self, _parent_transform: &Transform) {
        if !self.visible {
            return;
        }

        self.club_button.draw(&self.transform);
        self.diamond_button.draw(&self.transform);
        self.heart_button.draw(&self.transform);
        self.spade_button.draw(&self.transform);
    }
}
