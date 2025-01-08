use std::sync::mpsc::Sender;

use macroquad::math::Vec2;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

use crate::game::PlayerAction;
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

    pub dimmed_color: Color,
    pub dimmed: bool,

    pub trans_anim: Option<TranslationAnimator>,
    pub angle_anim: Option<RotationAnimator>,

    sender: Sender<PlayerAction>,
    pub player_action: Option<PlayerAction>,
}

impl CardView {
    pub fn new(id: u8, face: Texture2D, back: Texture2D, sender: Sender<PlayerAction>) -> Self {
        Self {
            id,
            transform: Transform::new(Vec2::ZERO, 0.0),
            card_image: Imager::new(face.clone(), 0.3333, true),
            eventer: Eventer::new(),
            face_texture: face,
            back_texture: back,
            dimmed_color: Color::from_rgba(200, 255, 200, 255),
            dimmed: false,
            trans_anim: None,
            angle_anim: None,
            sender,
            player_action: None,
        }
    }

    pub fn set_face_up(&mut self, face_up: bool) {
        self.card_image.texture = match face_up {
            true => self.face_texture.clone(),
            false => self.back_texture.clone(),
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

    /// Returns true if event found.
    pub fn process_events(&mut self, mouse_pos: &Vec2) -> bool {
        let size = self.card_image.draw_size();
        let centered = self.card_image.centered;

        match self
            .eventer
            .process_events(mouse_pos, &self.transform, size, centered)
        {
            Some(event) => {
                match event {
                    EventerEvent::LeftMouseReleased => {
                        // Send action if one exists. Dimmed cards should not have actions.
                        if let Some(action) = &self.player_action {
                            self.sender.send(action.clone()).expect("Send error");
                        }
                    }
                    _ => {}
                }
                // Regardless, card contained mouse_pos, so return true.
                return true;
            }
            None => {
                return false;
            }
        }
    }

    pub fn draw(&mut self) {
        if self.dimmed {
            self.card_image
                .draw(&self.transform, Some(self.dimmed_color));
        } else {
            self.card_image.draw(&self.transform, None);
        }
    }
}
