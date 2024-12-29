use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::card::SelectState;
use crate::view::eventer::Eventer;
use crate::view::eventer::EventerEvent;
use crate::view::imager::Imager;
use crate::view::transform::Transform;

use crate::view::animators::TranslationAnimator;

use super::animators::RotationAnimator;

pub struct CardView {
    pub id: u8, // must match Card id
    pub transform: Transform,
    pub card_image: Imager,
    pub eventer: Eventer,

    pub face_texture: Texture2D,
    pub back_texture: Texture2D,

    pub alt_color: Option<Color>,
    pub use_alt_color: bool,

    pub trans_anim: Option<TranslationAnimator>,
    pub angle_anim: Option<RotationAnimator>,

    pub select_state: SelectState,
}

impl CardView {
    pub fn new(id: u8, face: Texture2D, back: Texture2D) -> Self {
        Self {
            id,
            transform: Transform::new(Vec2::ZERO, 0.0),
            card_image: Imager::new(face.clone(), 0.3333, true),
            eventer: Eventer::new(),
            face_texture: face,
            back_texture: back,
            alt_color: Some(Color::from_rgba(200, 255, 200, 255)), //Some(Color::from_rgba(220, 220, 220, 255)),
            use_alt_color: false,
            trans_anim: None,
            angle_anim: None,
            select_state: SelectState::OutOfScope,
        }
    }

    pub fn set_face_up(&mut self, face_up: bool) {
        self.card_image.texture = match face_up {
            true => self.face_texture.clone(),
            false => self.back_texture.clone(),
        }
    }

    pub fn set_select_state(&mut self, state: SelectState) {
        self.select_state = state;
        self.use_alt_color = match self.select_state {
            SelectState::Selected => true,
            SelectState::Eligible => false,
            SelectState::Ineligible => false,
            SelectState::OutOfScope => false,
        }
    }

    pub fn move_to(&mut self, end_position: Vec2, velocity: f32) {
        self.trans_anim = Some(TranslationAnimator::new(
            self.transform.position,
            end_position,
            velocity,
        ));
    }

    pub fn rotate_to(&mut self, end_radian: f32, velocity: f32) {
        self.angle_anim = Some(RotationAnimator::new(
            self.transform.rotation,
            end_radian,
            velocity,
        ));
    }

    pub fn update(&mut self, time_delta: f32) {
        if let Some(translator) = &mut self.trans_anim {
            self.transform.position = translator.update(time_delta);
            if translator.completed {
                self.trans_anim = None;
            }
        }

        if let Some(rotator) = &mut self.angle_anim {
            self.transform.rotation = rotator.update(time_delta);
            if rotator.completed {
                self.angle_anim = None;
            }
        }
    }

    #[allow(dead_code)]
    pub fn contains_point(&self, point: &Vec2) -> bool {
        let size = self.card_image.draw_size();
        let centered = self.card_image.centered;
        self.eventer
            .contains_point(point, &self.transform, size, centered)
    }

    pub fn process_events(&mut self, mouse_pos: &Vec2) -> Option<EventerEvent> {
        let size = self.card_image.draw_size();
        let centered = self.card_image.centered;
        self.eventer
            .process_events(mouse_pos, &self.transform, size, centered)
    }

    pub fn draw(&mut self) {
        if self.use_alt_color {
            self.card_image.draw(&self.transform, self.alt_color);
        } else {
            self.card_image.draw(&self.transform, None);
        }
    }
}
